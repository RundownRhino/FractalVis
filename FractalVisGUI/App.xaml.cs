using System;
using System.Windows;
using System.Windows.Threading;

namespace FractalVisGUI
{
    /// <summary>
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App
    {
        private void App_OnDispatcherUnhandledException(object sender, DispatcherUnhandledExceptionEventArgs e) {
            Console.Write(@"Unhandled exception!");
            Console.Write(e.ToString());
            MessageBox.Show($"An unhandled exception occured: {e.Exception.Message}.", "Unhandled exception",
                MessageBoxButton.OK, MessageBoxImage.Error);
            //throw e.Exception; // Not needed as we don't set e.Handled?..
        }
    }
}