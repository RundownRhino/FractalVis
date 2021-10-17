using System.Windows;
using System.Windows.Media;
using System.Windows.Media.Imaging;

namespace FractalVisGUI
{
    internal static class Visualization
    {
        public static BitmapSource ArrToGrayscale(byte[] array, uint w, uint h) {
            var format = PixelFormats.Gray8;
            var wbm = new WriteableBitmap((int) w, (int) h, 96, 96, format,
                null); //TODO: use the other override to work with the pointer to array directly
            wbm.WritePixels(new Int32Rect(0, 0, (int) w, (int) h), array, (int) w, 0);
            return wbm;
        }

        public static BitmapSource ArrToRGB(byte[] array, uint w, uint h) {
            var format = PixelFormats.Rgb24;
            // var wbm = new WriteableBitmap((int) w, (int) h, 96, 96, format,
            //     null); //TODO: use the other override to work with the pointer to array directly
            // wbm.WritePixels(new Int32Rect(0, 0, (int) w, (int) h), array, (int) w, 0);
            // return wbm;
            var bitmap = BitmapSource.Create((int) w, (int) h, 96, 96, format, null, array, (int) w * 3);
            bitmap.Freeze();
            return bitmap;

            // //https://stackoverflow.com/questions/14337071/convert-array-of-bytes-to-bitmapimage
        }
    }
}