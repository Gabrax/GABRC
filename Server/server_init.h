#pragma once
#include <netinet/in.h>
#include <stdio.h>
#include <sys/socket.h>
#include <cstring>
#include <iostream>


struct HTTP_Server {
    int server_socket; 
    int port;
};

void init(HTTP_Server* server, int port) {
    server->port = port;

    
    server->server_socket = socket(AF_INET, SOCK_STREAM, 0);

    if (server->server_socket == -1) {
        perror("Socket error");
        exit(EXIT_FAILURE);
    }

    struct sockaddr_in address;
    address.sin_family = AF_INET;
    address.sin_port = htons(port);
    address.sin_addr.s_addr = INADDR_ANY;

   
    if (bind(server->server_socket, (struct sockaddr *)&address, sizeof(address)) < 0) {
        perror("Bind error");
        exit(EXIT_FAILURE);
    }

    
    if (listen(server->server_socket, 5) < 0) {
        perror("Listen error");
        exit(EXIT_FAILURE);
    }

    std::memset(address.sin_zero, '\0', sizeof address.sin_zero);

    std::cout << "Server Initialized " << server->port << '\n';
}

