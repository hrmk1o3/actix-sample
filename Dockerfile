FROM rust:latest
COPY . /root/
RUN cp /root/app/template.env /root/app/.env
CMD ["bash", "/root/app/run.sh"]
