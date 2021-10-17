using System;
using System.Runtime.InteropServices;

namespace FractalVisGUI
{
    // ReSharper disable once InconsistentNaming
    internal static class FFI
    {
        // ReSharper disable once InconsistentNaming
        public struct FFIVec
        {
            public unsafe byte* Ptr;
            public ulong Len;
            public ulong Cap;
        }

        [DllImport("fractal_calculator.dll")]
        public static extern FFIVec calculate_mandelbrot_vec(
            Double xMin,
            Double xMax,
            Double yMin,
            Double yMax,
            uint width,
            uint height,
            uint maxIters,
            Double horizon,
            byte shadesMax
        );

        [DllImport("fractal_calculator.dll")]
        public static extern FFIVec calculate_mandelbrot_vec_colored(
            Double xMin,
            Double xMax,
            Double yMin,
            Double yMax,
            uint width,
            uint height,
            uint maxIters,
            Double horizon,
            float fromAngle,
            float toAngle,
            float saturation
        );

        // ReSharper disable once InconsistentNaming
        public static byte[] FFIVecToArr(FFIVec vec) {
            var totLen = vec.Len;
            var temp = new byte[totLen];
            unsafe {
                Marshal.Copy(new IntPtr(vec.Ptr), temp, 0, (int) vec.Len);
            }

            return temp;
        }
    }
}