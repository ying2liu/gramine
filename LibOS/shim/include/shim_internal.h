/* SPDX-License-Identifier: LGPL-3.0-or-later */
/* Copyright (C) 2014 Stony Brook University */

#ifndef _SHIM_INTERNAL_H_
#define _SHIM_INTERNAL_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdnoreturn.h>

#include "api.h"
#include "assert.h"
#include "atomic.h"
#include "log.h"
#include "pal.h"
#include "pal_error.h"
#include "shim_defs.h"
#include "shim_internal-arch.h"
#include "shim_tcb.h"
#include "shim_types.h"

noreturn void* shim_init(int argc, const char** argv, const char** envp);

/* important macros and static inline functions */

extern int g_log_level;

extern struct pal_public_state* g_pal_public_state;

// TODO(mkow): We should make it cross-object-inlinable, ideally by enabling LTO, less ideally by
// pasting it here and making `inline`, but our current linker scripts prevent both.
void shim_log(int level, const char* fmt, ...) __attribute__((format(printf, 2, 3)));

#define DEBUG_HERE()                                             \
    do {                                                         \
        log_debug("%s (" __FILE__ ":%d)", __func__, __LINE__);   \
    } while (0)

/*!
 * \brief High-level syscall emulation entrypoint.
 *
 * \param context  CPU context at syscall entry.
 *
 * Emulates the syscall given the entry \p context.
 */
noreturn void shim_emulate_syscall(PAL_CONTEXT* context);

/*!
 * \brief Restore the CPU context.
 *
 * \param context  CPU context to restore.
 *
 * This function restores the given \p context. It is only called on returning from a syscall, so
 * it does not need to be reentrant (there is no such thing as nested syscalls), but it cannot
 * assume that the CPU context is the same as at the entry to the syscall (e.g. sigreturn, or signal
 * handling may change it).
 *
 * When AddressSanitizer is enabled, it also unpoisons the LibOS stack. Note at this point, we could
 * also be running from the initial PAL stack, but we unconditionally unpoison the LibOS stack for
 * simplicity.
 */
noreturn void return_from_syscall(PAL_CONTEXT* context);

/* Platform-specific part of return_from_syscall (called after ASan unpoisoning). */
noreturn void _return_from_syscall(PAL_CONTEXT* context);

/*!
 * \brief Restore the context after clone/fork.
 *
 * \param context  LibOS context to restore.
 *
 * Restores LibOS \p context after a successful clone or fork.
 */
noreturn void restore_child_context_after_clone(struct shim_context* context);

/*!
 * \brief Create a signal frame.
 *
 * \param context              CPU context.
 * \param siginfo              Signal to be delivered.
 * \param handler              Pointer to the user app signal handler.
 * \param restorer             Pointer to the restorer function.
 * \param should_use_altstack  `true` - use alternative stack if possible, `false` - never use it.
 * \param old_mask             Old signal mask (to be stored in the signal frame).
 *
 * Creates a signal frame on the user app stack (either normal or alternative stack, depending on
 * \p use_altstack and the currently used stack). Arranges \p context so that restoring it jumps
 * to \p handler with appropriate arguments and returning from \p handler will jump to \p restorer,
 * (which usually just calls `sigreturn` syscall). On most (all?) architectures old \p context,
 * \p siginfo and \p old_mask are saved into the signal frame.
 */
void prepare_sigframe(PAL_CONTEXT* context, siginfo_t* siginfo, void* handler, void* restorer,
                      bool should_use_altstack, __sigset_t* old_mask);

/*!
 * \brief Restart a syscall.
 *
 * \param context     CPU context.
 * \param syscall_nr  Syscall number.
 *
 * Arranges \p context so that upon return to it redoes the \p syscall_nr syscall.
 */
void restart_syscall(PAL_CONTEXT* context, uint64_t syscall_nr);

/*!
 * \brief Restore a sigreturn context.
 *
 * \param       Context original CPU context.
 * \param[out]  New_mask new signal mask.
 *
 * Restores CPU context in an architecture-specific way. On entry to this function \p context holds
 * initial CPU context and this function extracts signal frame (generated by `prepare_sigframe`)
 * and restores it into \p context. The signal mask extracted from the signal frame is written into
 * \p new_mask.
 */
void restore_sigreturn_context(PAL_CONTEXT* context, __sigset_t* new_mask);

/*!
 * \brief Emulate a syscall.
 *
 * \param context  CPU context.
 *
 * If the current instruction pointer in \p context points to a syscall instruction, arrange
 * \p context so that the syscall is emulated.
 * Returns `true` if it was a syscall instruction (hence a syscall will be emulated).
 * Note that this function merely changes context, so the actual emulation is done upon returning
 * to that context.
 * Used e.g. in Linux-SGX Pal to handle `syscall` instruction.
 */
bool maybe_emulate_syscall(PAL_CONTEXT* context);

/*!
 * \brief Handle a signal.
 *
 * \param context  CPU context.
 *
 * If there is a signal to be handled, this function arranges its delivery using `prepare_sigframe`.
 * Returns `true` if a not-ignored signal was handled (hence \p context was changed), `false`
 * otherwise.
 *
 * XXX: Signals are delivered only during transition from LibOS to the user app, so a situation is
 * possible when a signal is queued, but is not delivered for an arbitrary amount of time. There are
 * two distinct situations when this can happen:
 * 1) A blocking host-level syscall was issued and the signal arrived at any point before it and
 *    after the thread entered LibOS code. In such case the host syscall can block indefinitely.
 * 2) The signal arrived in the middle of or after `handle_signal`. In such case delivery of this
 *    signal is delayed until the next syscall is issued or another signal arrives.
 */
bool handle_signal(PAL_CONTEXT* context);

/*!
 * \brief Translate PAL error code into UNIX error code.
 *
 * The sign of the error code is preserved.
 */
long pal_to_unix_errno(long err);

void warn_unsupported_syscall(unsigned long sysno);
void debug_print_syscall_before(unsigned long sysno, ...);
void debug_print_syscall_after(unsigned long sysno, ...);

/* reference counter APIs */
#define REF_GET(ref)        __atomic_load_n(&(ref).counter, __ATOMIC_SEQ_CST)
#define REF_SET(ref, count) __atomic_store_n(&(ref).counter, count, __ATOMIC_SEQ_CST);

static inline int64_t __ref_inc(REFTYPE* ref) {
    int64_t _c;
    do {
        _c = __atomic_load_n(&ref->counter, __ATOMIC_SEQ_CST);
        assert(_c >= 0);
    } while (!__atomic_compare_exchange_n(&ref->counter, &_c, _c + 1, /*weak=*/false,
                                          __ATOMIC_SEQ_CST, __ATOMIC_RELAXED));
    return _c + 1;
}

#define REF_INC(ref) __ref_inc(&(ref))

static inline int64_t __ref_dec(REFTYPE* ref) {
    int64_t _c;
    do {
        _c = __atomic_load_n(&ref->counter, __ATOMIC_SEQ_CST);
        if (!_c) {
            log_error("Fail: Trying to drop reference count below 0");
            BUG();
            return 0;
        }
    } while (!__atomic_compare_exchange_n(&ref->counter, &_c, _c - 1, /*weak=*/false,
                                          __ATOMIC_SEQ_CST, __ATOMIC_RELAXED));
    return _c - 1;
}

#define REF_DEC(ref) __ref_dec(&(ref))

#ifndef __alloca
#define __alloca __builtin_alloca
#endif

#define ALLOC_ALIGNMENT         (g_pal_public_state->alloc_align)
#define IS_ALLOC_ALIGNED(x)     IS_ALIGNED_POW2(x, ALLOC_ALIGNMENT)
#define IS_ALLOC_ALIGNED_PTR(x) IS_ALIGNED_PTR_POW2(x, ALLOC_ALIGNMENT)
#define ALLOC_ALIGN_DOWN(x)     ALIGN_DOWN_POW2(x, ALLOC_ALIGNMENT)
#define ALLOC_ALIGN_UP(x)       ALIGN_UP_POW2(x, ALLOC_ALIGNMENT)
#define ALLOC_ALIGN_DOWN_PTR(x) ALIGN_DOWN_PTR_POW2(x, ALLOC_ALIGNMENT)
#define ALLOC_ALIGN_UP_PTR(x)   ALIGN_UP_PTR_POW2(x, ALLOC_ALIGNMENT)

void* __system_malloc(size_t size);
void __system_free(void* addr, size_t size);

#define system_malloc __system_malloc
#define system_free   __system_free

extern void* migrated_memory_start;
extern void* migrated_memory_end;

static inline bool memory_migrated(void* mem) {
    return mem >= migrated_memory_start && mem < migrated_memory_end;
}

extern void* __load_address;
extern void* __load_address_end;

extern const char** migrated_envp;

int init_brk_region(void* brk_region, size_t data_segment_size);
void reset_brk(void);
int init_rlimit(void);

bool is_user_memory_readable(const void* addr, size_t size);
bool is_user_memory_writable(const void* addr, size_t size);
bool is_user_string_readable(const char* addr);

uint64_t get_rlimit_cur(int resource);
void set_rlimit_cur(int resource, uint64_t rlim);

int object_wait_with_retry(PAL_HANDLE handle);

struct shim_handle;

/*!
 * \brief Interrupt all threads waiting on epolls which \p handle is associated with.
 *
 * \param handle  Handle to wakeup waiters of.
 *
 * Currently the only usage of this function is on socket handles that were just connected (in
 * which case their PAL handle changed from NULL to a newly created handle).
 */
void interrupt_epolls(struct shim_handle* handle);

/*!
 * \brief Delete all epoll items associated with the pair \p fd and \p handle
 *
 * \param fd      FD which was just closed/detached.
 * \param handle  Handle which \p fd referred to.
 *
 * This should be called once there is no possibility of adding new epoll items with this \p fd and
 * \p handle, i.e. after \p fd was detached from fds map.
 */
void delete_epoll_items_for_fd(int fd, struct shim_handle* handle);

/*!
 * \brief Check if next `epoll_wait` with `EPOLLET` should trigger for this handle.
 *
 * \param handle       Handle to check.
 * \param ret          Return value from last operation.
 * \param in           `true` if last operation was of input type (e.g. `read`).
 * \param was_partial  `true` if last operation did not fill the whole buffer.
 *
 * This function should be called after each in/out operation on \p handle that is epoll-able,
 * i.e. that you can wait on using `epoll` syscall. \p ret should be the return value from that
 * operation, \p in should indicate if that was an input operation (e.g. `read` or `accept`) or
 * output (e.g. `write`), \p was_partial - if that operation was done partially (e.g. `read` did not
 * fill the whole buffer).
 *
 * This `EPOLLET` emulation is not entirely identical with Linux. Unfortunately we cannot implement
 * it ideally without PAL support, but adding such support is impossible on PALs other than native
 * Linux. Due to this we have a "hacky" approach: if a handle is successfully waited for on some
 * epoll instance using `EPOLLET` semantics, it's marked as not epollet-ready and then is not
 * included in further `EPOLLET` waits. To mark it back as epollet-ready, an operation must be
 * observed, which either returns `-EAGAIN` (non-blocking operation which cannot be completed atm)
 * or a partial operation (e.g. `read` not filling the whole buffer). The idea is that application
 * cannot assume that all data was processed (hence expect next `EPOLLET` wait not to hang), until
 * it sees one of the above happening.
 */
void maybe_epoll_et_trigger(struct shim_handle* handle, int ret, bool in, bool was_partial);

void* allocate_stack(size_t size, size_t protect_size, bool user);
int init_stack(const char** argv, const char** envp, const char*** out_argp, elf_auxv_t** out_auxv);

/*!
 * \brief Jump to the defined entry point.
 *
 * \param entry  Address defined in the elf entry point.
 * \param argp   Pointer to the initial stack, contains program arguments and environment.
 *
 * This function does not return.
 *
 * The implementation of this function depends on the used architecture.
 */
noreturn void call_elf_entry(elf_addr_t entry, void* argp);

#endif /* _SHIM_INTERNAL_H_ */
