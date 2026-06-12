# rust-scanner
Port scanning tool developed in Rust. It is conceptually similar to Nmap, but built from scratch in Rust without relying on Nmap at all. The purpose of this project is twofold. This rust-scanner will be customized to my bug bounty workflow and will also help to reinforce the concepts that I am learning in The Rust Book and my Rustlings exercises. The code will be written by me manually and will not be vibe coded. I am using Claude.ai to assist me on my learning journey. I use Claude to help me work through problems/questions and assist with documentation, design decisions, and organization of my workflow. My goal is to learn Rust and to practice what I am learning by creating a program on my own. 

## Basic Features
Port scanning  
Service fingerprinting  
Basic vulnerability checks  
Output in multiple formats  
Integration with bug bounty workflows  

## How it grows with my skills:  
### Starting simple:

Scan a single IP for open ports  
Basic TCP connect scan  
Print results to terminal  

### As I learn more Rust:

Add concurrent scanning with threads  
Add timeout handling  
Accept command line arguments  
Save results to a file  

### More advanced:  

Service detection and banner grabbing  
Multiple target support  
Output formatting (JSON, CSV)  
Eventually async scanning with Tokio  
