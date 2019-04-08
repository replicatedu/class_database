FROM ubuntu

ENV ROCKET_ADDRESS 0.0.0.0

RUN apt update 
RUN apt install -y git  

RUN mkdir root/.ssh
ADD deploy_key root/.ssh/id_rsa
ADD class_database root/

RUN git config --global user.email "STUDENT_DATABASE"
RUN git config --global user.name "STUDENT_DATABASE"

RUN chmod 0700 /root/.ssh/id_rsa


WORKDIR root/

