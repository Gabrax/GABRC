#include <iostream>
#include <sys/socket.h>
#include <unistd.h>
#include <stdlib.h>
#include <netinet/in.h>
#include <string>

int main()
{

    int server_fd= socket(AF_INET,SOCK_STREAM,0);

    if(server_fd= socket(AF_INET,SOCK_STREAM,0) < 0){
        perror("fuck you");
        return 0;
    }



    return 0;
}
