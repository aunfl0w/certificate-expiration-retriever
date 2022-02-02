# certificate-expiration-retriever
Retrieve the expiration date of an X509 certificate from a host's certificate handshake

```
certificate-expiration-retriever 1.0.0
Sean Hardwick <aun.sswick@gmail.com>
Retrieve the expiration date of an X509 certificate from a host's certificate handshake. Direct
network connectivity to the host is required.

OUTPUT host | days valid | expiration date

USAGE:
    certificate-expiration-retriever [OPTIONS] --host <HOST>

OPTIONS:
    -h, --host <HOST>
            FQDN of host

        --help
            Print help information

    -p, --port <PORT>
            [default: 443]

    -V, --version
            Print version information


$ ./certificate-expiration-retriever --host expired.badssl.com
expired.badssl.com | -2487.02 | Sun, 12 Apr 2015 23:59:59 +0000 

```
