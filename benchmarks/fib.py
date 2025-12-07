#!/usr/bin/env python3
# Fibonacci benchmark - recursive implementation
# Tests function call overhead and recursion performance

def fib(n):
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)

print(fib(35))
