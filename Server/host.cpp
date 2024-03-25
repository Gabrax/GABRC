#include <iostream>
#include <unistd.h>
#include <stdlib.h>
#include <string>
#include <cstring>
#include <fstream>
#include <sstream>

#include <sys/socket.h>
#include <netinet/in.h>
#include <sys/types.h>

#define port 2137

int main()
{

    int server_socket, client_socket; 
    

    // Read HTML file
    FILE *html;
    html = fopen("/home/gabrax/Desktop/github/HTTP_Server/Website/index.html", "r");
    if(html == NULL) {
        perror("Error opening html");
        exit(EXIT_FAILURE); 
    }

    char response[1024];
    fgets(response,1024,html);

    char http_header[2048] = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
    strcat(http_header, response);

    if((server_socket = socket(AF_INET,SOCK_STREAM,0)) == 0){
        perror("Socket error");
        exit(EXIT_FAILURE);
    }

    struct sockaddr_in address;
    address.sin_family = AF_INET;
    address.sin_port = htons(port);
    address.sin_addr.s_addr = INADDR_ANY;

    //std::memset(address.sin_zero, '\0', sizeof address.sin_zero);

    if(bind(server_socket, (struct sockaddr *) &address, sizeof(address)) < 0){
        perror("Bind error");
        exit(EXIT_FAILURE);
    }
    if(listen(server_socket, 5) < 0){
        perror("Listen error");
        exit(EXIT_FAILURE);
    }

    //int addrlen = sizeof(address);
    while (1)
    {
        puts("\n--------- Waiting for connection ---------\n\n");
        
        if((client_socket = accept(server_socket,NULL,NULL)) < 0){
            perror("Accepting failed");
            exit(EXIT_FAILURE);
        }


        if(send(client_socket,http_header,sizeof(http_header), 0) < 0){
            perror("Send failed");
        }

        puts("------------HTML Sent------------\n");
        close(client_socket);
    }

    return 0;
}
