using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Data;
using System.Linq;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;
using System.Windows.Media.Imaging;
using F = System.Double;

namespace FractalVisGUI
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow

    {
        private F _xMin = -2f;
        private F _xMax = 2f;
        private F _yMin = -2f;
        private F _yMax = 2f;


        private void RecalculateImage() {
            var (width, height) = Get_Viewport_Size();
            if (
                width < 1 || height < 1 ||
                double.IsNaN(width) || double.IsNaN(height)) {
                return;
            }

            uint maxIters;
            double horizon;
            byte shadesMax;
            uint roots_count;
            FractalKind kind;
            try {
                maxIters = uint.Parse(MaxIters.Text);
                horizon = double.Parse(Horizon.Text);
                shadesMax = byte.Parse(Shades.Text);
                roots_count = uint.Parse(PolyRootNumber.Text);
                kind = FractalKind.Of(FractalKindChosen.Text); //FIXME
            }
            catch (Exception e) when (e is FormatException || e is OverflowException) {
                Console.WriteLine($@"Failed to parse values: {e.Message}, {e.TargetSite}.");
                return;
            }

            Console.WriteLine($@"Recalculating to viewport of x: {_xMin:F} to {_xMax:F}, y: {_yMin:F} to {_yMax:F}.");
            BitmapSource bitmap;
            if (kind == FractalKind.MandelbrotColored) {
                float fromAngle = 0;
                float toAngle = 300;
                float saturation = 1;
                var vec = FFI.FFIVecToArr(FFI.calculate_mandelbrot_vec_colored(
                    _xMin, _xMax, _yMin, _yMax,
                    (uint) width, (uint) height,
                    maxIters, horizon,
                    fromAngle, toAngle, saturation));
                bitmap = Visualization.ArrToRGB(vec, (uint) width, (uint) height);
            }
            else if (kind == FractalKind.MandelbrotGrayscale) {
                var vec = FFI.FFIVecToArr(FFI.calculate_mandelbrot_vec(
                    _xMin, _xMax, _yMin, _yMax,
                    (uint) width, (uint) height,
                    maxIters, horizon,
                    shadesMax));
                bitmap = Visualization.ArrToGrayscale(vec, (uint) width, (uint) height);
            }
            else if (kind == FractalKind.Newton) {
                float fromAngle = 0;
                float toAngle = 300;
                float saturation = 1;
                var vec = FFI.FFIVecToArr(FFI.calculate_newton_roots_of_unity_vec(
                    _xMin, _xMax, _yMin, _yMax,
                    (uint) width, (uint) height,
                    maxIters, horizon,
                    roots_count,
                    fromAngle, toAngle, saturation));
                bitmap = Visualization.ArrToRGB(vec, (uint) width, (uint) height);
            }
            else {
                throw new ArgumentException("This should be impossible - kind is not of one of the handled types.");
            }


            ImageViewer1.Source = bitmap;
        }

        public MainWindow() {
            InitializeComponent();
            var outputter = new TextBoxOutputter(TestBox);
            Console.SetOut(outputter);
            RecalculateImage();
        }

        private bool _mouseDown; // Set to 'true' when mouse is held down.
        private Point _mouseDownPos; // The point where the mouse button was clicked down

        private void Grid_MouseDown(object sender, MouseButtonEventArgs e) {
            if (e.ChangedButton == MouseButton.Middle) {
                _xMin = -2;
                _xMax = 2;
                _yMin = -2;
                _yMax = 2;
                RecalculateImage();
                return;
            }


            // Capture and track the mouse.
            _mouseDown = true;
            _mouseDownPos = e.GetPosition(ImageCanvas);
            theGrid.CaptureMouse();

            // Initial placement of the drag selection box.         
            Canvas.SetLeft(selectionBox, _mouseDownPos.X);
            Canvas.SetTop(selectionBox, _mouseDownPos.Y);
            selectionBox.Width = 0;
            selectionBox.Height = 0;

            // Make the drag selection box visible.
            selectionBox.Visibility = Visibility.Visible;
        }

        private (double width, double height) Get_Viewport_Size() {
            var width = ImageViewer1.Width;
            var height = ImageViewer1.Height;
            return (width, height);
        }

        private void Grid_MouseUp(object sender, MouseButtonEventArgs e) {
            if (e.ChangedButton == MouseButton.Middle) {
                return;
            }

            var (width, height) = Get_Viewport_Size();

            if (
                width < 1 || height < 1 ||
                double.IsNaN(width) || double.IsNaN(height)) {
                return;
            }

            // Release the mouse capture and stop tracking it.
            _mouseDown = false;
            theGrid.ReleaseMouseCapture();

            // Hide the drag selection box.
            selectionBox.Visibility = Visibility.Collapsed;

            Point mouseUpPos = e.GetPosition(ImageCanvas);
            var xPosMin = Math.Min(_mouseDownPos.X, mouseUpPos.X);
            var xPosMax = Math.Max(_mouseDownPos.X, mouseUpPos.X);
            var yPosMin = Math.Min(_mouseDownPos.Y, mouseUpPos.Y);
            var yPosMax = Math.Max(_mouseDownPos.Y, mouseUpPos.Y);
            double newXMin, newXMax, newYMax, newYMin;
            switch (e.ChangedButton) {
                case MouseButton.Left:
                    newXMin = xPosMin / width * (_xMax - _xMin) + _xMin;
                    newXMax = xPosMax / width * (_xMax - _xMin) + _xMin;
                    newYMin = yPosMin / height * (_yMax - _yMin) + _yMin;
                    newYMax = yPosMax / height * (_yMax - _yMin) + _yMin;
                    break;
                case MouseButton.Right:
                    newXMin = (xPosMax * _xMin - xPosMin * _xMax) / (xPosMax - xPosMin);
                    newXMax = (xPosMax * _xMin - xPosMin * _xMax + width * (_xMax - _xMin)) / (xPosMax - xPosMin);
                    newYMin = (yPosMax * _yMin - yPosMin * _yMax) / (yPosMax - yPosMin);
                    newYMax = (yPosMax * _yMin - yPosMin * _yMax + height * (_yMax - _yMin)) / (yPosMax - yPosMin);
                    break;
                default:
                    return;
            }

            _xMin = newXMin;
            _xMax = newXMax;
            _yMin = newYMin;
            _yMax = newYMax;

            RecalculateImage();
        }

        private void Grid_MouseMove(object sender, MouseEventArgs e) {
            if (!_mouseDown) return;
            // When the mouse is held down, reposition the drag selection box.

            Point mousePos = e.GetPosition(ImageCanvas);

            if (_mouseDownPos.X < mousePos.X) {
                Canvas.SetLeft(selectionBox, _mouseDownPos.X);
                selectionBox.Width = mousePos.X - _mouseDownPos.X;
            }
            else {
                Canvas.SetLeft(selectionBox, mousePos.X);
                selectionBox.Width = _mouseDownPos.X - mousePos.X;
            }

            if (_mouseDownPos.Y < mousePos.Y) {
                Canvas.SetTop(selectionBox, _mouseDownPos.Y);
                selectionBox.Height = mousePos.Y - _mouseDownPos.Y;
            }
            else {
                Canvas.SetTop(selectionBox, mousePos.Y);
                selectionBox.Height = _mouseDownPos.Y - mousePos.Y;
            }
        }

        private void MaxIters_OnPreviewTextInput(object sender, TextCompositionEventArgs e) {
            e.Handled = !uint.TryParse(((TextBox) sender).Text + e.Text, out var a) || a > 1000;
        }

        private void Horizon_OnPreviewTextInput(object sender, TextCompositionEventArgs e) {
            e.Handled = !double.TryParse(((TextBox) sender).Text + e.Text, out _);
        }

        private void OnParamInputChanged(object sender, KeyEventArgs keyEventArgs) {
            if (keyEventArgs.Key != Key.Enter) {
                return;
            }

            OnParamChanged();
        }

        private void OnParamChanged() {
            RecalculateImage();
        }

        private void Shades_OnPreviewTextInput(object sender, TextCompositionEventArgs e) {
            e.Handled = !byte.TryParse(((TextBox) sender).Text + e.Text, out _);
        }

        private void Colored_OnToggle(object sender, RoutedEventArgs e) {
            OnParamChanged();
        }
    }

    internal class FractalKinds : ObservableCollection<string>
    {
        public FractalKinds() {
            foreach (var x in FractalKind.FractalKinds) {
                Add(x.Name);
            }
        }
    }

    class FractalKind
    {
        public readonly string Name;

        public FractalKind(string name) {
            this.Name = name;
        }

        public static readonly FractalKind MandelbrotGrayscale = new FractalKind("Mandelbrot (grayscale)");
        public static readonly FractalKind MandelbrotColored = new FractalKind("Mandelbrot (colored)");
        public static readonly FractalKind Newton = new FractalKind("Newton");

        public static readonly List<FractalKind> FractalKinds = new List<FractalKind> {
            MandelbrotGrayscale, MandelbrotColored, Newton
        };

        public static FractalKind Of(string name) => FractalKinds.First(x => x.Name == name);

        protected bool Equals(FractalKind other) {
            return Name == other.Name;
        }

        public override int GetHashCode() {
            return (Name != null ? Name.GetHashCode() : 0);
        }
    }
}