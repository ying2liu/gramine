ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

CFLAGS = -Wall -Wextra

ifeq ($(DEBUG),1)
GRAMINE_LOG_LEVEL = debug
CFLAGS += -g
else
GRAMINE_LOG_LEVEL = error
CFLAGS += -O3
endif

.PHONY: all
all: enclave-agent.manifest async-client.manifest
ifeq ($(SGX),1)
all: enclave-agent.manifest.sgx enclave-agent.sig enclave-agent.token async-client.manifest.sgx async-client.sig async-client.token
endif

enclave-agent.manifest: enclave-agent.manifest.template
	gramine-manifest \
                -Dlog_level=$(GRAMINE_LOG_LEVEL) \
                -Dexecdir=$(shell dirname $(shell which enclave-agent)) \
                -Darch_libdir=$(ARCH_LIBDIR) \
                $< >$@

async-client.manifest: async-client.manifest.template
	gramine-manifest \
                -Dlog_level=$(GRAMINE_LOG_LEVEL) \
                -Dexecdir=$(shell dirname $(shell which async-client)) \
                -Darch_libdir=$(ARCH_LIBDIR) \
                $< >$@

enclave-agent.sig enclave-agent.manifest.sgx: sgx_sign_agent
        @:
async-client.sig async-client.manifest.sgx: sgx_sign_client
        @:

.INTERMEDIATE: sgx_sign
sgx_sign_agent: enclave-agent.manifest enclave-agent
	gramine-sgx-sign \
                --manifest $< \
                --output $<.sgx
sgx_sign_client: async-client.manifest async-client
	gramine-sgx-sign \
                --manifest $< \
                --output $<.sgx


enclave-agent.token: enclave-agent.sig
	gramine-sgx-get-token --output enclave-agent.token --sig enclave-agent.sig
async-client.token: async-client.sig
	gramine-sgx-get-token --output async-client.token --sig async-client.sig

ifeq ($(SGX),)
GRAMINE = gramine-direct
else
GRAMINE = gramine-sgx
endif

.PHONY: clean
clean:
	$(RM) *.token *.sig *.manifest.sgx *.manifest OUTPUT
.PHONY: distclean
distclean: clean
