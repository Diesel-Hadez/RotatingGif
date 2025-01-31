
import init, { greet , get_stuff } from "./pkg/hello_wasm.js";
import { initBuffers } from "./init-buffers.js";
import { drawScene } from "./draw-scene.js";
function createImageFromTexture(gl, texture, width, height) {
    // Create a framebuffer backed by the texture
    var framebuffer = gl.createFramebuffer();
    gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);

    // Read the contents of the framebuffer
    var data = new Uint8Array(width * height * 4);
    gl.readPixels(0, 0, width, height, gl.RGBA, gl.UNSIGNED_BYTE, data);

    gl.deleteFramebuffer(framebuffer);

    // Create a 2D canvas to store the result 
    var canvas = document.createElement('canvas');
    canvas.width = width;
    canvas.height = height;
    var context = canvas.getContext('2d');

    // Copy the pixels to a 2D canvas
    var imageData = context.createImageData(width, height);
    imageData.data.set(data);
    context.putImageData(imageData, 0, 0);

    var img = new Image();
    img.src = canvas.toDataURL();
    return img;
}

init().then(() => {
  // fetch("sample_2_animation.gif", {
  fetch("neko-dance.gif", {
    method: 'GET',
  }).then((response) => {
    console.log(response)
    response.arrayBuffer().then((ab) => {
    let hmm = new Uint8Array(ab);
    console.log(hmm);
  let example = get_stuff(hmm);
  console.log(JSON.stringify(example));
      let blob_image = `P3\n${example['width']} ${example['height']}\n255\n`;

      for (let i = 0; i < example.width*example.height;i+=1 ){
        blob_image += `${example.frames[0].data[i*4]} ${example.frames[0].data[i*4+1]} ${example.frames[0].data[i*4+2]}\n`;
      }
      blob_image = URL.createObjectURL(new Blob([blob_image], {type: "image/x-portable-pixmap"}));
      const link = document.createElement("a");
      link.href = blob_image;
      link.setAttribute('download', "gif_parsed_image_first_frame.ppm");
      // link.click();
      console.log(blob_image);
  // greet("WebAssembly");
function loadTexture(gl, num) {
  const texture = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, texture);
  const level = 0;
  const internalFormat = gl.RGBA;
  const width = example.width;
  const height = example.height;
  const border = 0;
  const srcFormat = gl.RGBA;
  const srcType = gl.UNSIGNED_BYTE;
  const pixel = example['frames'][num]['data'];
  // const pixel = new Uint8Array([
  //   0, 0, 255, 255, // opaque blue
  //   0,  255, 0, 255, // opaque green 
  //   255, 0, 0, 255, // opaque red 
  //   0, 0, 255, 255, // opaque blue
  //
  // ]);   
//   pixel = new Uint8Array(
//     Array((50*50)/2).fill([
//     0,0,255,255
//   ]).flat()
// .concat(
//     Array((50*50)/2).fill([
//     0,255,0,255
//     ]).flat()
//   ));
  console.log(pixel.length)
  gl.texImage2D(
    gl.TEXTURE_2D,
    level,
    internalFormat,
    width,
    height,
    border,
    srcFormat,
    srcType,
    pixel,
  );
  
  // gl.generateMipmap(gl.TEXTURE_2D);
      // gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      // gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
  
  function isPowerOf2(value) {
    return (value & (value - 1)) === 0;
  }

  return texture;
}

  //
  // Initialize a shader program, so WebGL knows how to draw our data
  //
  function initShaderProgram(gl, vsSource, fsSource) {
    const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
    const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

    // Create the shader program

    const shaderProgram = gl.createProgram();
    gl.attachShader(shaderProgram, vertexShader);
    gl.attachShader(shaderProgram, fragmentShader);
    gl.linkProgram(shaderProgram);

    // If creating the shader program failed, alert

    if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
      alert(
        `Unable to initialize the shader program: ${gl.getProgramInfoLog(
          shaderProgram,
        )}`,
      );
      return null;
    }

    return shaderProgram;
  }

  //
  // creates a shader of the given type, uploads the source and
  // compiles it.
  //
  function loadShader(gl, type, source) {
    const shader = gl.createShader(type);

    // Send the source to the shader object

    gl.shaderSource(shader, source);

    // Compile the shader program

    gl.compileShader(shader);

    // See if it compiled successfully

    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      alert(
        `An error occurred compiling the shaders: ${gl.getShaderInfoLog(shader)}`,
      );
      gl.deleteShader(shader);
      return null;
    }

    return shader;
  }

const main = () => {
  const canvas = document.getElementById("glcanvas");
  const gl = canvas.getContext("webgl2");
  if (gl === null) {
    alert(
      "Unable to initialize WebGL. Your browser or machine may not support it.",
    );
    return;
  }
  const vsSource = `#version 300 es
    layout (location = 0) in vec4 aVertexPosition;
    layout (location = 1) in vec2 aTextureCoord;

    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;

    out highp vec2 vTextureCoord;

    void main() {
      gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
      vTextureCoord = aTextureCoord;
    }
  `;
  const fsSource = `#version 300 es
    in highp vec2 vTextureCoord;
    out lowp vec4 fragColor;

    uniform sampler2D uSampler;
    void main() {
      // fragColor = mix(texture(uSampler, vTextureCoord), vec4(0,255,0,255), 0.1);

      // fragColor = vec4((vTextureCoord.y * vec3(1,0,0)).xyz, 1);
      highp vec4 t = texture(uSampler, vTextureCoord);
      if (t.a < 0.1) {
        discard;
      }
      fragColor = t;
    }
  `;

  // Initialize a shader program; this is where all the lighting
  // for the vertices and so forth is established.
  const shaderProgram = initShaderProgram(gl, vsSource, fsSource);
  const programInfo = {
    program: shaderProgram,
    attribLocations: {
      vertexPosition: gl.getAttribLocation(shaderProgram, "aVertexPosition"),
      textureCoord: gl.getAttribLocation(shaderProgram, "aTextureCoord"),
    },
    uniformLocations: {
      projectionMatrix: gl.getUniformLocation(shaderProgram, "uProjectionMatrix"),
      modelViewMatrix: gl.getUniformLocation(shaderProgram, "uModelViewMatrix"),
      uSampler: gl.getUniformLocation(shaderProgram, "uSampler"),
    },
  };

  // Here's where we call the routine that builds all the
// objects we'll be drawing.
const buffers = initBuffers(gl);

let cur_frame = 0;
let texture = loadTexture(gl, cur_frame);
const nexter = () => {
  setTimeout(() => {
    texture = loadTexture(gl, cur_frame++);
    cur_frame = cur_frame % example.frames.length;
    nexter();
  }, (example.frames[cur_frame].delay < 1 ? 1 : example.frames[cur_frame].delay) *10);
}
nexter();

document.body.appendChild(createImageFromTexture(gl,texture,example.width, example.height));


let then = 0;
let rotation = 0;

// Draw the scene
const render = (now) => {
  now *= 0.001
  let deltaTime = now - then
  then = now
  // Set clear color to black, fully opaque
  gl.clearColor(0.5961, 0.8314, 0.7333, 1.0);
  // Clear the color buffer with specified clear color
  gl.clear(gl.COLOR_BUFFER_BIT);
  rotation += deltaTime;
  drawScene(gl, programInfo, texture, buffers, rotation);
  requestAnimationFrame(render);
}
requestAnimationFrame(render)


}
main();

});
  });

    });
