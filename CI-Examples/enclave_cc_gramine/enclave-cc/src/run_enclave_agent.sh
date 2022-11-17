#!/bin/bash
set -e
# compile enclave-agent
pushd enclave-agent
#cargo build --example enclave-agent --release
#make
make release
#make debug
popd
# initialize occlum workspace
rm -rf occlum_instance && occlum new occlum_instance && pushd occlum_instance
#new_json="$(jq '.resource_limits.user_space_size = "8000MB" |
#            .resource_limits.kernel_space_heap_size= "5000MB" |
#            .resource_limits.kernel_space_stack_size= "500MB" |
#            .resource_limits.max_num_of_threads = 32 |
#           .metadata.debuggable = false ' Occlum.json)" && \
#echo "${new_json}" > Occlum.json
new_json="$(jq '.resource_limits.user_space_size = "1024MB" |
            .resource_limits.kernel_space_heap_size= "512MB" |
            .resource_limits.kernel_space_stack_size= "128MB" |
            .resource_limits.max_num_of_threads = 16 |
            .env.default += ["http_proxy=http://proxy-chain.intel.com:911"] |            .env.default += ["https_proxy=http://proxy-chain.intel.com:912"] |
            .metadata.debuggable = false ' Occlum.json)" && \
echo "${new_json}" > Occlum.json
#new_json="$(jq '.resource_limits.user_space_size = "256MB" |
#            .resource_limits.kernel_space_heap_size= "256MB" |
#            .resource_limits.kernel_space_stack_size= "4MB" |
#            .resource_limits.max_num_of_threads = 16 |
#           .metadata.debuggable = true ' Occlum.json)" && \
#echo "${new_json}" > Occlum.json
rm -rf image
copy_bom -f ../enclave-agent.yaml --root image --include-dir /opt/occlum/etc/template
#copy_bom -f ../enclave-agent-1.yaml --root image --include-dir /opt/occlum/etc/template
occlum build
#mkdir -p images/scratch-base_v1.8/sefs/lower
#mkdir -p images/scratch-base_v1.8/sefs/upper
#sudo mkdir -p /images/yingtest_v1/sefs/lower
#sudo mkdir -p /images/yingtest_v1/sefs/upper

#echo "hello"
#sudo /opt/occlum/build/bin/occlum run /bin/enclave-agent
##!/bin/bash
set -e
# compile enclave-agent
pushd enclave-agent
#cargo build --example enclave-agent --release
#make
make release
#make debug
popd
# initialize occlum workspace
rm -rf occlum_instance && occlum new occlum_instance && pushd occlum_instance
#new_json="$(jq '.resource_limits.user_space_size = "8000MB" |
#            .resource_limits.kernel_space_heap_size= "5000MB" |
#            .resource_limits.kernel_space_stack_size= "500MB" |
#            .resource_limits.max_num_of_threads = 32 |
#           .metadata.debuggable = false ' Occlum.json)" && \
#echo "${new_json}" > Occlum.json
new_json="$(jq '.resource_limits.user_space_size = "1024MB" |
            .resource_limits.kernel_space_heap_size= "512MB" |
            .resource_limits.kernel_space_stack_size= "128MB" |
            .resource_limits.max_num_of_threads = 16 |
            .env.default += ["http_proxy=http://proxy-chain.intel.com:911"] |            .env.default += ["https_proxy=http://proxy-chain.intel.com:912"] |
            .metadata.debuggable = false ' Occlum.json)" && \
echo "${new_json}" > Occlum.json
#new_json="$(jq '.resource_limits.user_space_size = "256MB" |
#            .resource_limits.kernel_space_heap_size= "256MB" |
#            .resource_limits.kernel_space_stack_size= "4MB" |
#            .resource_limits.max_num_of_threads = 16 |
#           .metadata.debuggable = true ' Occlum.json)" && \
#echo "${new_json}" > Occlum.json
rm -rf image
copy_bom -f ../enclave-agent.yaml --root image --include-dir /opt/occlum/etc/template
#copy_bom -f ../enclave-agent-1.yaml --root image --include-dir /opt/occlum/etc/template
occlum build
#mkdir -p images/scratch-base_v1.8/sefs/lower
#mkdir -p images/scratch-base_v1.8/sefs/upper
#sudo mkdir -p /images/yingtest_v1/sefs/lower
#sudo mkdir -p /images/yingtest_v1/sefs/upper

#echo "hello"
#sudo /opt/occlum/build/bin/occlum run /bin/enclave-agent
#sudo occlum run /bin/enclave-agent
#OCCLUM_LOG_LEVEL=trace occlum run /bin/enclave-agent
#occlum package   occlum_instance.tar.gz
popd
#sudo occlum run /bin/enclave-agent
#OCCLUM_LOG_LEVEL=trace occlum run /bin/enclave-agent
#occlum package   occlum_instance.tar.gz
popd
