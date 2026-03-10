package com.example.java_api;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;

// Java we use Controllers
@RestController
public class HelloController {
    
    @Autowired
    private GreetingService greetingService;

    @GetMapping("/greet/{name}")
    public ResponseEntity<String> greet(@PathVariable String name) {
        if (name == null || name.trim().isEmpty()) {
            return ResponseEntity
                .status(HttpStatus.BAD_REQUEST)
                .body("Erro: Name connot be empyt!");
        }

        String successMensage = greetingService.saveAndGreet(name.trim());
        return ResponseEntity.ok(successMensage);
    }
}