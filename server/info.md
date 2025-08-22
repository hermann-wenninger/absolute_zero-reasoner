AUSGABE im BROWSER durch: http://127.0.0.1:7878 

Server läuft auf http://127.0.0.1:7878 ...

Anfrage erhalten:
GET / HTTP/1.1
Host: 127.0.0.1:7878
Connection: keep-alive
sec-ch-ua: "Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138"
sec-ch-ua-mobile: ?0
sec-ch-ua-platform: "Windows"
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br, zstd
Accept-Language: de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7
Cookie: _ga=GA1.1.1009269672.1745127542; _ga_TKBR7CECXS=GS1.1.1745127541.1.1.1745127604.58.0.0; __stripe_mid=e5ee4828-7c2c-4a3a-b31c-97cfa9ed3e031606a2; csrftoken=P1GUbWiFdPLEcbggFKU0EZVKeUJCpC5L

AUSGABE von ANFRAGE durch: curl -v http://127.0.0.1:7878

*   Trying 127.0.0.1:7878...
* Connected to 127.0.0.1 (127.0.0.1) port 7878
* using HTTP/1.x
> GET / HTTP/1.1
> Host: 127.0.0.1:7878
> User-Agent: curl/8.14.1
> Accept: */*
> 
< HTTP/1.1 200 OK
< Content-Type: text/plain; charset=UTF-8
< Content-Length: 13
< 
Hallo, Welt!* abort upload
* end of response with 1 bytes missing
* closing connection #0
curl: (18) end of response with 1 bytes missing