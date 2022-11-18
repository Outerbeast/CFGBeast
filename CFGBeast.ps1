<# CFGBeast 1.0 
by Outerbeast #>
Add-Type -Name Window -Namespace Console -MemberDefinition '
[DllImport("Kernel32.dll")]
public static extern IntPtr GetConsoleWindow();
[DllImport("user32.dll")]
public static extern bool ShowWindow(IntPtr hWnd, Int32 nCmdShow);
'
$consolePtr = [Console.Window]::GetConsoleWindow()
[Console.Window]::ShowWindow( $consolePtr, 0 )

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName PresentationCore,PresentationFramework

enum WriteType
{
    OVERWRITE
    APPEND
    REMOVE
};

$strVersion = "1.0"
$blInputAccepted = $false
$iCFGsWritten = 0
$strAppTitle = "CFGBeast $strVersion"

$Host.UI.RawUI.WindowTitle = $strAppTitle

$g_Window = [System.Windows.Forms.Form]
$g_Label = [System.Windows.Forms.Label]
$g_Button = [System.Windows.Forms.Button]
$g_TextBox = [System.Windows.Forms.TextBox]

$gui = New-Object $g_Window
$gui.ClientSize = '595,315' # Window size
$gui.Text =  $strAppTitle # Title
$gui.BackColor = "#ffffff"
$gui.FormBorderStyle = 'FixedSingle'
$gui.StartPosition = "CenterScreen"
$gui.AutoSizeMode = 'GrowAndShrink'
$gui.ShowInTaskbar = $true

$label = New-Object $g_Label
$label.Text = "Enter your CFG CVars in the box below:"
$label.Location = New-Object System.Drawing.Size( 10, 10 )
$label.Size = New-Object System.Drawing.Size( 280, 20 )
$label.AutoSize = $true

$textBox = New-Object $g_TextBox
$textBox.Location = New-Object System.Drawing.Size( 10, 40 )
$textBox.Size = New-Object System.Drawing.Size( 575, 230 )
$textBox.AcceptsReturn = $true
$textBox.AcceptsTab = $false
$textBox.Multiline = $true
$textBox.ScrollBars = 'Both'
#$textBox.Text = "mp_npckill 2"

$btnCreate = New-Object $g_Button
$btnCreate.Text = 'Create'
$btnCreate.Location = New-Object System.Drawing.Size( 10, 280 )
$btnCreate.Size = New-Object System.Drawing.Size( 75, 25 )
$btnCreate.Add_Click( { $gui.Tag = $textBox.Text; CreateCfg -WriteType 'OVERWRITE' } )

$btnAdd = New-Object $g_Button
$btnAdd.Text = 'Add'
$btnAdd.Location = New-Object System.Drawing.Size( 105, 280 )
$btnAdd.Size = New-Object System.Drawing.Size( 75, 25 )
$btnAdd.Add_Click( { $gui.Tag = $textBox.Text; CreateCfg -WriteType 'APPEND' } )

$btnRemove = New-Object $g_Button
$btnRemove.Text = 'Remove'
$btnRemove.Location = New-Object System.Drawing.Size( 200, 280 )
$btnRemove.Size = New-Object System.Drawing.Size( 75, 25 )
$btnRemove.Add_Click( { $gui.Tag = $textBox.Text; CreateCfg -WriteType 'REMOVE' } )

$btnHelp = New-Object $g_Button
$btnHelp.Text = '?'
$btnHelp.Location = New-Object System.Drawing.Size( 559, 10 )
$btnHelp.Size = New-Object System.Drawing.Size( 25, 25 )
$btnHelp.Add_Click( { ShowHelp } )

$btnDelete = New-Object $g_Button
$btnDelete.Location = New-Object System.Drawing.Size( 415, 280 )
$btnDelete.Size = New-Object System.Drawing.Size( 75, 25 )
$btnDelete.Text = "Delete"
$btnDelete.Add_Click( { DeleteCfg; $global:blInputAccepted = $false } )

$btnCancel = New-Object $g_Button
$btnCancel.Location = New-Object System.Drawing.Size( 510, 280 )
$btnCancel.Size = New-Object System.Drawing.Size( 75, 25 )
$btnCancel.Text = "Cancel"
$btnCancel.Add_Click( { $gui.Tag = $null; $gui.Close(); $global:blInputAccepted = $false } )

$gui.Controls.AddRange( @( $label, $textBox, $btnCreate, $btnAdd, $btnRemove, $btnCancel, $btnDelete, $btnHelp ) )

function ShowHelp
{
    $strCredit = "CFGBeast Version $global:strVersion`nCreated by Outerbeast`n`n"
    $strHelpInfo1 = "Generate CFG files for your all your maps automatically.`nLaunch the application in your map folder and input your desired CVars in the box.`nRefer to default_map_settings.cfg for CVars you can use for your maps."
    $strHelpInfo2 = "`n`n-Usage Information-`nCreate: creates new CFG files with the CVars set in the input box`nAdd: appends CVars to existing CFGs`nRemove: deletes the CVar from the CFG`nDelete: deletes all CFG files from the current folder"
    $strThanks = "`n`nThank you for using this app!`nIf you'd like to give feedback feel free to put them here: https://github.com/Outerbeast/CFGBeast/issues"
    [System.Windows.MessageBox]::Show( "$strCredit$strHelpInfo1$strHelpInfo2$strThanks", "Help", "OK", "Question" )
}

function CreateCfg([WriteType] $writetype)
{
    $strCurrentPath = [System.Environment]::CurrentDirectory
    $strCVarIn = $gui.Tag

    if( $strCVarIn )
    {
        $global:blInputAccepted = $true

        Get-ChildItem -Path $strCurrentPath -Filter *.bsp | Foreach-Object {

            $cfgname = $_.BaseName
            
            switch( $writetype )
            {
                'OVERWRITE'
                {
                    New-Item -Name "$cfgname.cfg" -Path $strCurrentPath -ItemType "file" -Value $strCVarIn -Force
                    $global:iCFGsWritten++
                }
                
                'APPEND'
                {
                    Add-Content -Path "$strCurrentPath\$cfgname.cfg" "`r`n$strCVarIn"
                    $global:iCFGsWritten++
                }

                'REMOVE'
                {
                    ( Get-Content $strCurrentPath\$cfgname.cfg ).replace( $strCVarIn, '' ) | Set-Content $strCurrentPath\$cfgname.cfg
                    $global:iCFGsWritten++
                }

                Default {}
            }
        }
    }
    else
    { 
        [System.Windows.MessageBox]::Show( "You did not add in any CVars.`nEnter your CVars in the text box and try again.", "No CVars specified", "OK", "Warning" )
        $global:blInputAccepted = $false
    }

    if( $global:blInputAccepted )
    {
        $gui.Close()
    }
}

function DeleteCfg
{
    $strConfirmDelete = [System.Windows.MessageBox]::Show( "Are you sure you want to delete all CFG files?`nThis action cannot be undone.", "Confirm Deletion", "YesNo", "Warning" )

    if( $strConfirmDelete -eq "Yes" )
    {
        $strCurrentPath = [System.Environment]::CurrentDirectory
        Remove-Item $strCurrentPath\*.cfg
        [System.Windows.MessageBox]::Show( "All CFG files were deleted.", "Done", "OK", "Information" )
    }
}

$gui.ShowDialog()

if( $global:blInputAccepted )
{
    if( $global:iCFGsWritten )
    {
        [System.Windows.MessageBox]::Show( "$iCFGsWritten CFG files written.", "Done", "OK", "Information" )
    }
    else
    {
        [System.Windows.MessageBox]::Show( "No CFG files written.`n`nPlease place the app executable in a map folder with valid BSPs and try again.", "ERROR", "OK", "Error" )
    }
}

$gui.Dispose()
exit
# For Debug.
#Read-Host -Prompt "Press Enter to exit"