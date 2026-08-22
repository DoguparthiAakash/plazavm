using System;
using System.IO;
using System.IO.Compression;
using System.Reflection;
using System.Windows.Forms;
using System.Diagnostics;

namespace PlazaVMInstaller
{
    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                string appData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
                string installDir = Path.Combine(appData, "PlazaVM");
                string binDir = Path.Combine(installDir, "bin");

                // Read resource
                Assembly assembly = Assembly.GetExecutingAssembly();
                using (Stream resStream = assembly.GetManifestResourceStream("PlazaVM_Payload.zip"))
                {
                    if (resStream == null)
                    {
                        MessageBox.Show("Installer payload missing from executable!", "PlazaVM Installer Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                        return;
                    }

                    if (Directory.Exists(installDir))
                    {
                        Directory.Delete(installDir, true);
                    }
                    Directory.CreateDirectory(installDir);

                    using (ZipArchive archive = new ZipArchive(resStream))
                    {
                        archive.ExtractToDirectory(installDir);
                    }
                }

                // Update PATH
                string userPath = Environment.GetEnvironmentVariable("PATH", EnvironmentVariableTarget.User) ?? "";
                if (!userPath.Contains(binDir))
                {
                    string newPath = userPath.TrimEnd(';') + ";" + binDir;
                    Environment.SetEnvironmentVariable("PATH", newPath, EnvironmentVariableTarget.User);
                }

                MessageBox.Show("PlazaVM has been installed successfully!\n\nOpen a new terminal and type 'plaza --help' to get started.", 
                    "PlazaVM Installer", MessageBoxButtons.OK, MessageBoxIcon.Information);
            }
            catch (Exception ex)
            {
                MessageBox.Show("Installation failed:\n" + ex.Message, "PlazaVM Installer Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }
    }
}
