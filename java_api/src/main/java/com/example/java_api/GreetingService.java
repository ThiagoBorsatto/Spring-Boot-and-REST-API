package com.example.java_api;

import org.springframework.stereotype.Service;

@Service
public class GreetingService {
    public String generateGreeting(String name) {
        return "Hello, " + name +"!";
    }
}
