.data
    /* Define your data variables here */
    my_var: .word 0x12345678    // Example of a 32-bit word variable

.text
    .global _start
_start:
    MOV X0, #5        // Load 5 into X0
    MOV X1, #10       // Load 10 into X1
    ADD X2, X0, X1    // Add X0 and X1, store result in X2

    /* System call to exit */
    MOV X8, #93       // Load system call number for exit into X8
    MOV X16, #0       // Load exit status into X16
    SVC #0            // Make system call to exit
