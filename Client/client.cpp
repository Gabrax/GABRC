#include <iostream>
#include <string>
#include <cstring>

#include <sys/socket.h>
#include <unistd.h>
#include <stdlib.h>
#include <netinet/in.h>
#include <arpa/inet.h>

#define port 2137

int main()
{
    int sock = 0; 
    struct sockaddr_in serv_adrr;

    std::string hello = "Hello from client\n";

    char buffer[1024] = {0};
    if((sock = socket(AF_INET, SOCK_STREAM,0)) < 0){
        puts("\n Socket creation error \n");
        return -1;
    }

    std::memset(&serv_adrr, '0', sizeof(serv_adrr));

    serv_adrr.sin_family = AF_INET;
    serv_adrr.sin_port = htons(port);

    if(inet_pton(AF_INET,"127.0.0.1",&serv_adrr.sin_addr)<=0){
        puts("\nInvalid address/ Address not supported \n");
        return -1;
    }
    
    if(connect(sock,(struct sockaddr *)&serv_adrr, sizeof(serv_adrr)) < 0){
        puts("\n Connection failed \n");
        return -1;
    }

    send(sock,hello.c_str(),hello.size(),0);
    puts("Hello message sent\n");
    
    #pragma GCC diagnostic ignored "-Wunused-variable"
    long long valread = read(sock,buffer,1024);
    std::cout << buffer << '\n';


    return 0;
}
