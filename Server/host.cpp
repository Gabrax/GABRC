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

#include "server_init.h"
#include "routing.h"

#define port 2137

int main()
{
    HTTP_Server server;
    init(&server,port);
    

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

    std::shared_ptr<Route> route = initRoute("/", "index.html");
    route = addRoute(route, "/yoo", "yoo.html");
    inorder(route);

    //int addrlen = sizeof(address);
    int client_socket; 
    while (1)
    {
        puts("\n--------- Waiting for connection ---------\n\n");
        
        if((client_socket = accept(server.server_socket,NULL,NULL)) < 0){
            perror("Accepting failed");
            exit(EXIT_FAILURE);
        }

        char buffer[30000] = {0};
        #pragma GCC diagnostic ignored "-Wunused-variable"
        long long valread = read(client_socket,buffer,sizeof(buffer));
        std::cout << buffer << '\n';    

        if(send(client_socket,http_header,sizeof(http_header), 0) < 0){
            perror("Send failed");
        }else{
            puts("------------ HTML Sent ------------\n");
        }

        close(client_socket);
    }

    return 0;
}
