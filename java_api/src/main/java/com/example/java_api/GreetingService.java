package com.example.java_api;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class GreetingService {

    @Autowired
    private UserRepository userRepository;

    public String saveAndGreet(String name) {
        Users newUser = new Users();
        newUser.setName(name);
        userRepository.save(newUser);

        return "Saved " + name + " to the database and said Hello!";
    }
}
