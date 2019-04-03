FROM ubuntu

RUN apt update 
RUN apt install -y git  

RUN mkdir root/.ssh
ADD deploy_key root/.ssh/id_rsa
ADD class_database root/

RUN git config --global user.email "STUDENT_DATABASE"
RUN git config --global user.name "STUDENT_DATABASE"

WORKDIR root/

