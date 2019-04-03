rm -rf docker_context
rm -rf database
~/.cargo/bin/cargo build
mkdir -p docker_context
mkdir -p database
cp deploy_key docker_context/
cp Dockerfile docker_context/
cp target/debug/class_database docker_context/
docker build -t db_test_client ./docker_context/.
docker run -it \
--mount type=bind,source="$(pwd)"/database,target=/tmp \
 db_test_client /root/class_database 