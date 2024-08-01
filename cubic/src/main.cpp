#include <math.h>
#include <iostream>
#ifndef _WIN32
#include <unistd.h>
#else
#include <windows.h>
void usleep(__int64 usec)
{
  HANDLE timer;
  LARGE_INTEGER ft;

  ft.QuadPart = -(10 * usec); // Convert to 100 nanosecond interval, negative value indicates relative time

  timer = CreateWaitableTimer(NULL, TRUE, NULL);
  SetWaitableTimer(timer, &ft, 0, NULL, NULL, 0);
  WaitForSingleObject(timer, INFINITE);
  CloseHandle(timer);
}
#endif
using namespace std;

class Cube {
public:
  Cube(int w, int h, int d, int cw): width(w), height(h), area(w*h), distanceFromCam(d), cubeWidth(cw) {
    zBuffer = new float[area];
    buffer = new char[area];
  }
  ~Cube() {
    delete[] zBuffer, buffer;
  }

  void process() {
    cout << "\x1b[2J";
    while (1) {
      memset(buffer, backgroundASCIICode, area);
      memset(zBuffer, 0, area * 4);

      for (float cubeX = -cubeWidth; cubeX < cubeWidth; cubeX += incrementSpeed) {
        for (float cubeY = -cubeWidth; cubeY < cubeWidth;
            cubeY += incrementSpeed) {
          calculateForSurface(cubeX, cubeY, -cubeWidth, '@');
          calculateForSurface(cubeWidth, cubeY, cubeX, '$');
          calculateForSurface(-cubeWidth, cubeY, -cubeX, '~');
          calculateForSurface(-cubeX, cubeY, cubeWidth, '#');
          calculateForSurface(cubeX, -cubeWidth, -cubeY, ';');
          calculateForSurface(cubeX, cubeWidth, cubeY, '+');
        }
      }
      
      cout << "\x1b[H";
      for (int k = 0; k < area; k++) {
        cout << (char)(k % width ? buffer[k] : 10);
      }

      A += 0.05;
      B += 0.05;
      C += 0.01;
      usleep(16000);
    }
  }

protected:
  float calculateX(int i, int j, int k) {
    return j * sin(A) * sin(B) * cos(C) - k * cos(A) * sin(B) * cos(C) +
          j * cos(A) * sin(C) + k * sin(A) * sin(C) + i * cos(B) * cos(C);
  }

  float calculateY(int i, int j, int k) {
    return j * cos(A) * cos(C) + k * sin(A) * cos(C) -
          j * sin(A) * sin(B) * sin(C) + k * cos(A) * sin(B) * sin(C) -
          i * cos(B) * sin(C);
  }

  float calculateZ(int i, int j, int k) {
    return k * cos(A) * cos(B) - j * sin(A) * cos(B) + i * sin(B);
  }

  void calculateForSurface(float cubeX, float cubeY, float cubeZ, int ch) {
    float x = calculateX(cubeX, cubeY, cubeZ);
    float y = calculateY(cubeX, cubeY, cubeZ);
    float z = calculateZ(cubeX, cubeY, cubeZ) + distanceFromCam;

    float ooz = 1 / z;

    int xp = (int)(width / 2 + K1 * ooz * x * 2);
    int yp = (int)(height / 2 + K1 * ooz * y);

    int idx = xp + yp * width;
    if (idx >= 0 && idx < area) {
      if (ooz > zBuffer[idx]) {
        zBuffer[idx] = ooz;
        buffer[idx] = ch;
      }
    }
  }

private:
  float A, B, C;

  float cubeWidth;
  int width, height, area;
  float *zBuffer;
  char *buffer;
  int backgroundASCIICode = '_';
  int distanceFromCam;
  float incrementSpeed = 0.57;
  float K1 = 40;
};

int main() {
  Cube *c = new Cube(110, 44, 100, 20);
  c->process();
  delete c;
  return 0;
}