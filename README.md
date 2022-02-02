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

# dependencies
From ubuntu 22.04
* libssl3

```
$ ldd target/release/certificate-expiration-retriever 
	linux-vdso.so.1 (0x00007ffd3c5e9000)
	libssl.so.3 => /lib/x86_64-linux-gnu/libssl.so.3 (0x00007ff555d4c000)
	libcrypto.so.3 => /lib/x86_64-linux-gnu/libcrypto.so.3 (0x00007ff55590b000)
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007ff5558f1000)
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007ff55580d000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007ff5555e5000)
	/lib64/ld-linux-x86-64.so.2 (0x00007ff555f27000)
$ dpkg -S libssl.so.3
libssl3:amd64: /usr/lib/x86_64-linux-gnu/libssl.so.3
$ dpkg -S libcrypto.so.3
libssl3:amd64: /usr/lib/x86_64-linux-gnu/libcrypto.so.3

```
