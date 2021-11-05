FROM fluent/fluent-bit:1.8

COPY ./conf/fluent-bit.conf /fluent-bit/etc/
COPY conf/parsers.conf /fluent-bit/etc/

CMD ["/fluent-bit/bin/fluent-bit","-c","/fluent-bit/etc/fluent-bit.conf"]
