docker rmi -f db_test_client
rm -rf docker_context
rm -rf database
~/.cargo/bin/rustup default nightly
~/.cargo/bin/cargo build
mkdir -p docker_context
mkdir -p database
cp /tmp/id_rsa docker_context/deploy_key
cp Dockerfile docker_context/
cp target/debug/class_database docker_context/
~/.cargo/bin/rustup default stable
docker build -t db_test_client ./docker_context/.
docker run -it -p 8000:8000\
 --mount type=bind,source="$(pwd)"/database,target=/tmp \
 db_test_client /root/class_database 