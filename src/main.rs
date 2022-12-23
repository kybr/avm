// Karl Yerkes
// 2022-12-22
//

use byteorder::{ByteOrder, LittleEndian};
use cpal::traits::{DeviceTrait, HostTrait};
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
    // execute a program that accepts an int (sample count) and spurts
    // an array of that many f32 (sample values).
    let mut child = Command::new("binary-sine")
        .arg("110") // 
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    //println!("{:?}", child);

    let mut command = child
        .stdin
        .take() // was .as_mut()
        .ok_or("Child process stdin has not been captured!")?;
    let mut result = child
        .stdout
        .take()
        .ok_or("Child process stdout has not been captured!")?;

    let mut buffer_char = [0u8; 1024 * std::mem::size_of::<f32>()];

    // XXX add command line stuff to select io device configuration
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let config = device.default_output_config().unwrap();
    println!("Default output config : {:?}", config);

    let _stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            command.write_all(b"1024\n");
            result.read(&mut buffer_char); // XXX handle errors?
            LittleEndian::read_f32_into(&buffer_char, data);
        },
        move |_err| {
            // XXX handle errors
        },
    );

    // wait for the user to type something
    // XXX replace with signal handling and maybe a little command REPL
    println!("Enter to quit.");
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();

    //child.kill().unwrap(); // XXX do better

    Ok(()) // whatever this means
}

/*

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

double sine(double x) {  // expects x on (-1, 1) !!!!!
  double xx = x * x;
  return x * (3.138982 + xx * (-5.133625 + xx * (2.428288 - xx * 0.433645f)));
}

int main(int argc, char* argv[]) {
  float frequency = 440.0;
  if (argc > 1) {
    frequency = atof(argv[1]);
  }
  char char_buffer[10];
  float *float_buffer = malloc(10000 * sizeof(float));
  float value = 0;
  while (1) {
    scanf("%s", char_buffer);
    int n = atoi(char_buffer);
    if (argc > 2) fprintf(stderr, "got %d\n", n);
    for (int i = 0; i < n; i++) {
      float_buffer[i] = 0.99 * sine(2 * value - 1);
      value += frequency / 44100;
      if (value >= 1.0) {
        value -= 1.0;
      }
    }
    int result = write(fileno(stdout), float_buffer, n * sizeof(float));
    if (argc > 2) fprintf(stderr, "\nwrote %d\n", result);
  }
}

*/
