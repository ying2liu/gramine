#SGX_SIGNER_KEY ?= /home/vdhanraj/.config/gramine/enclave-key.pem
SGX_SIGNER_KEY ?= /home/yliu79/.config/gramine/enclave-key.pem
GRAMINEDIR ?= ../..
GRAMINE_PKGLIBDIR ?= /usr/lib/x86_64-linux-gnu/gramine # this is debian/ubuntu specific

ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

execdir=`pwd`

ifeq ($(DEBUG),1)
GRAPHENE_LOG_LEVEL = debug
else
GRAPHENE_LOG_LEVEL = error
endif

GRAPHENE_LOG_LEVEL = debug

.PHONY: all
all: agent_app agent_app.manifest app_app app_app.manifest
ifeq ($(SGX),1)
all: agent_app.manifest.sgx agent_app.sig agent_app.token  app_app.manifest.sgx app_app.sig app_app.token
endif

CFLAGS += -g -Wall -std=c11 -I$(GRAMINEDIR)/pal/src/host/linux-sgx/tools/common \
			-I$(GRAMINEDIR)/common/src/protected_files  \
			-I$(GRAMINEDIR)/tools/sgx/common \
          $(shell pkg-config --cflags mbedtls_gramine)
LDFLAGS += $(shell pkg-config --libs mbedtls_gramine) \
			-L/home/vdhanraj/Gramine-dev/enclave-cc/ying-poc/gramine/build/tools/sgx/common/ -lsgx_util \
			-L/home/vdhanraj/Gramine-dev/enclave-cc/ying-poc/gramine/build/subprojects/mbedtls-mbedtls-2.26.0-2/ -lmbedcrypto_gramine \
			-L/usr/lib/x86_64-linux-gnu -lcurl



agent_app:
	$(CC) agent_app.c comm_lib.c $(CFLAGS) $(LDFLAGS) -Wl,--no-as-needed -lsgx_urts -lmbedcrypto_gramine -pthread -o agent_app
app_app:
	$(CC) app_app.c comm_lib.c $(CFLAGS) $(LDFLAGS) -Wl,--no-as-needed -lsgx_urts -lmbedcrypto_gramine -pthread -o app_app


agent_app.manifest: agent_app.manifest.template
	gramine-manifest \
		-Dlog_level=$(GRAPHENE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		$< > $@

app_app.manifest: app_app.manifest.template
	gramine-manifest \
		-Dlog_level=$(GRAPHENE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		$< > $@

agent_app.manifest.sgx: agent_app.manifest agent_app
	@test -s $(SGX_SIGNER_KEY) || \
	    { echo "SGX signer private key was not found, please specify SGX_SIGNER_KEY!"; exit 1; }
	gramine-sgx-sign \
		--key $(SGX_SIGNER_KEY) \
		--manifest $< \
		--output $@
	gramine-sgx-get-token \
		--output agent_app.token --sig agent_app.sig

app_app.manifest.sgx: app_app.manifest app_app
	@test -s $(SGX_SIGNER_KEY) || \
	    { echo "SGX signer private key was not found, please specify SGX_SIGNER_KEY!"; exit 1; }
	gramine-sgx-sign \
		--key $(SGX_SIGNER_KEY) \
		--manifest $< \
		--output $@
	gramine-sgx-get-token \
		--output app_app.token --sig app_app.sig


.PHONY: clean
clean:
	$(RM) agent_app app_app *.token *.sig *.manifest.sgx *.manifest

.PHONY: distclean
distclean: clean
	$(RM) agent_app app_ap agent_app_debug.log app_app_debug.log *.token *.sig *.manifest.sgx *.manifest

