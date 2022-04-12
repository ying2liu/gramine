SGX_SIGNER_KEY ?= ../../Pal/src/host/Linux-SGX/signer/enclave-key.pem

ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

execdir=`pwd`

ifeq ($(DEBUG),1)
GRAPHENE_LOG_LEVEL = debug
else
GRAPHENE_LOG_LEVEL = error
endif

GRAPHENE_LOG_LEVEL = debug

.PHONY: all
all: server server.manifest
ifeq ($(SGX),1)
all: server.manifest.sgx server.sig server.token
endif



server.manifest: server.manifest.template
	gramine-manifest \
		-Dlog_level=$(GRAPHENE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		$< > $@

server.manifest.sgx: server.manifest server
	@test -s $(SGX_SIGNER_KEY) || \
	    { echo "SGX signer private key was not found, please specify SGX_SIGNER_KEY!"; exit 1; }
	gramine-sgx-sign \
		--key $(SGX_SIGNER_KEY) \
		--manifest $< \
		--output $@
	gramine-sgx-get-token \
		--output server.token --sig server.sig


.PHONY: clean
clean:
	$(RM) *.token *.sig *.manifest.sgx *.manifest 

.PHONY: distclean
distclean: clean
	$(RM) *.token *.sig *.manifest.sgx *.manifest 

