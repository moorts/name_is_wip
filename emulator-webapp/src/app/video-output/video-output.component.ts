import { HttpClient } from '@angular/common/http';
import { Component, ElementRef, Input, ViewChild, AfterViewInit } from '@angular/core';
import { EmulatorService } from '../emulator-service/emulator.service';

@Component({
  selector: 'video-output',
  templateUrl: './video-output.component.html',
  styleUrls: ['./video-output.component.scss']
})
export class VideoOutputComponent implements AfterViewInit {

  @Input() public width: number = 256;
  @Input() public height: number = 224;
  @Input() public scale: number = 1.0;

  @ViewChild("canvas") private canvas!: ElementRef<HTMLCanvasElement>;

  private gl!: WebGL2RenderingContext;
  private frameTexture: WebGLTexture | null = null;
  private shaderProgram: WebGLProgram | null = null;
  private vertSource: string | undefined;
  private fragSource: string | undefined;
  private vao: WebGLVertexArrayObject | null = null;
  private textureLocation: WebGLUniformLocation | null = null;
  private scaleLocation: WebGLUniformLocation | null = null;

  constructor(private readonly emulatorService: EmulatorService, private readonly httpClient: HttpClient) {
    httpClient.get("assets/shaders/simple.frag", {responseType: 'text'}).subscribe(result => {
      this.fragSource = result;
      this.initProgram();
    });
    httpClient.get("assets/shaders/simple.vert", {responseType: 'text'}).subscribe(result => {
      this.vertSource = result;
      this.initProgram();
    });
  }

  ngAfterViewInit(): void {
    const gl = this.canvas.nativeElement.getContext("webgl2");
    if (gl) {
      this.gl = gl;
      this.initGraphics();

    } else {
      console.log("Unable to get WebGL 2 context.");
    }
  }

  private initGraphics() {
    const gl = this.gl;
    this.frameTexture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, this.frameTexture);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, 1, 1, 0, gl.RGBA, gl.UNSIGNED_BYTE,
      new Uint8Array([20, 255, 20, 255]));
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.bindTexture(gl.TEXTURE_2D, null);
  }

  private initProgram() {
    if (!this.fragSource || !this.vertSource) return;

    const gl = this.gl;

    const vertShader = this.compileShader(gl, this.vertSource, this.gl.VERTEX_SHADER);
    const fragShader = this.compileShader(gl, this.fragSource, this.gl.FRAGMENT_SHADER);

    this.shaderProgram = this.createProgram(gl, vertShader, fragShader);

      // look up where the vertex data needs to go.
    var positionAttributeLocation = gl.getAttribLocation(this.shaderProgram, "a_position");

    // lookup uniforms
    this.textureLocation = gl.getUniformLocation(this.shaderProgram, "u_texture");
    this.scaleLocation = gl.getUniformLocation(this.shaderProgram, "u_scale");

    // Create a vertex array object (attribute state)
    this.vao = gl.createVertexArray();

    // and make it the one we're currently working with
    gl.bindVertexArray(this.vao);

    // create the position buffer, make it the current ARRAY_BUFFER
    // and copy in the color values
    const positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    // Put a unit quad in the buffer
    const positions = [
      -1, -1,
      -1, 1,
      1, -1,
      1, -1,
      -1, 1,
      1, 1,
    ];
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

    // Turn on the attribute
    gl.enableVertexAttribArray(positionAttributeLocation);

    // Tell the attribute how to get data out of positionBuffer (ARRAY_BUFFER)
    var size = 2;          // 2 components per iteration
    var type = gl.FLOAT;   // the data is 32bit floats
    var normalize = false; // don't normalize the data
    var stride = 0;        // 0 = move forward size * sizeof(type) each iteration to get the next position
    var offset = 0;        // start at the beginning of the buffer
    gl.vertexAttribPointer(
        positionAttributeLocation, size, type, normalize, stride, offset);
  }

  public update() {
    const gl = this.gl;
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
    gl.clearColor(0, 0, 0, 0);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

    if (!this.emulatorService.running) return;

    // Update texture
    const data = this.emulatorService.memory;

    // Upload data to a WebGL texture that only has a red channel
    gl.bindTexture(gl.TEXTURE_2D, this.frameTexture);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.R8, this.width / 8, this.height, 0, gl.RED, gl.UNSIGNED_BYTE, data);

    if (!this.shaderProgram) return;

    gl.useProgram(this.shaderProgram);

    // Setup the attributes for the quad
    gl.bindVertexArray(this.vao);

    const textureUnit = 0;
    // the shader we're putting the texture on texture unit 0
    gl.uniform1i(this.textureLocation, textureUnit);
    gl.uniform1f(this.scaleLocation, this.scale);

    // Bind the texture to texture unit 0
    gl.activeTexture(gl.TEXTURE0 + textureUnit);
    gl.bindTexture(gl.TEXTURE_2D, this.frameTexture);

    // draw the quad (2 triangles, 6 vertices)
    gl.drawArrays(gl.TRIANGLES, 0, 6);
  }

    /**
   * Creates and compiles a shader.
   *
   * @param {!WebGLRenderingContext} gl The WebGL Context.
   * @param {string} shaderSource The GLSL source code for the shader.
   * @param {number} shaderType The type of shader, VERTEX_SHADER or
   *     FRAGMENT_SHADER.
   * @return {!WebGLShader} The shader.
   */
  private compileShader(gl: WebGL2RenderingContext, shaderSource: string, shaderType: number) {
    // Create the shader object
    const shader = gl.createShader(shaderType)!;

    // Set the shader source code.
    gl.shaderSource(shader, shaderSource);

    // Compile the shader
    gl.compileShader(shader);

    // Check if it compiled
    var success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
    if (!success) {
      // Something went wrong during compilation; get the error
      throw ("could not compile shader:" + gl.getShaderInfoLog(shader));
    }

    return shader;
  }

  /**
   * Creates a program from 2 shaders.
   *
   * @param {!WebGLRenderingContext) gl The WebGL context.
   * @param {!WebGLShader} vertexShader A vertex shader.
   * @param {!WebGLShader} fragmentShader A fragment shader.
   * @return {!WebGLProgram} A program.
   */
  private createProgram(gl: WebGL2RenderingContext, vertexShader: WebGLShader, fragmentShader: WebGLShader) {
    // create a program.
    var program = gl.createProgram()!;

    // attach the shaders.
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);

    // link the program.
    gl.linkProgram(program);

    // Check if it linked.
    var success = gl.getProgramParameter(program, gl.LINK_STATUS);
    if (!success) {
        // something went wrong with the link; get the error
        throw ("program failed to link:" + gl.getProgramInfoLog(program));
    }

    return program;
  };

}
