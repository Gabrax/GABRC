#include <iostream>
#include <memory> 
#include <string> 

struct Route {
    std::string key;
    std::string value;
    std::shared_ptr<Route> left;
    std::shared_ptr<Route> right;
};

std::shared_ptr<Route> initRoute(const std::string& key, const std::string& value) {
    auto temp = std::make_shared<Route>();

    temp->key = key;
    temp->value = value;

    temp->left = nullptr;
    temp->right = nullptr;
    return temp;
}

void inorder(const std::shared_ptr<Route>& root) {
    if (root != nullptr) {
        inorder(root->left);
        std::cout << root->key << " -> " << root->value << std::endl;
        inorder(root->right);
    }
}

std::shared_ptr<Route> addRoute(std::shared_ptr<Route> root, const std::string& key, const std::string& value) {
    if (root == nullptr) {
        return initRoute(key, value);
    }

    if (key == root->key) {
        std::cout << "============ WARNING ============\n";
        std::cout << "A Route For \"" << key << "\" Already Exists\n";
    } else if (key > root->key) {
        root->right = addRoute(root->right, key, value);
    } else {
        root->left = addRoute(root->left, key, value);
    }

    return root;
}

std::shared_ptr<Route> search(const std::shared_ptr<Route>& root, const std::string& key) {
    if (root == nullptr) {
        return nullptr;
    }

    if (key == root->key) {
        return root;
    } else if (key > root->key) {
        return search(root->right, key);
    } else { // key < root->key
        return search(root->left, key);
    }
}

