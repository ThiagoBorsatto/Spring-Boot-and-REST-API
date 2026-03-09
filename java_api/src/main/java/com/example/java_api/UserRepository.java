package com.example.java_api;

import org.springframework.data.jpa.repository.JpaRepository;

public interface UserRepository extends JpaRepository<Users, Long> {
    // Spring create all the CRUD for the User object!
}
