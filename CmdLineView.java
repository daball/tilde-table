import java.io.File;
import java.util.ArrayList;
import java.util.Scanner;

/**
 * CmdLineView creates the command-line prompt view for the application.
 * 
 * To use it, call runLoop().
 * 
 * @author David Ball
 */
public class CmdLineView
{
    /**
     * Runs the main program view loop. User must type exit to break out.
     * 
     * @param  args   any command-line args
     * @return     0, no errors
     */
    public static int runLoop(String[] args)
    {
        //print help page
        printHelp();
        //loop on the command-line
        String commandLine = "";
        while (!commandLine.startsWith("exit") && !commandLine.startsWith("quit"))
        {
            commandLine = promptUser("itec441# ").trim();
            if (commandLine != "")
            {
                dispatchCommandLine(commandLine);
            }
        }
        //print exit message
        System.out.println("Goodbye!");
        return 0;
    }
    
    /**
     * Dispatches/routes any command provided to the proper handler.
     */
    private static void dispatchCommandLine(String commandLine)
    {
        if (commandLine == null || commandLine.trim().length() == 0)
        {
            //short circuit, no command entered, nothing to dispatch
            return;
        }
        else if (commandLine.startsWith("exit") || commandLine.startsWith("quit"))
        {
            //short circuit, nothing to dispatch, but this should break any loop pending an exit
            return;
        }
        else if (commandLine.startsWith("help") || commandLine.startsWith("?"))
        {
            printHelp();
        }
        else if (commandLine.startsWith("ver"))
        {
            printBanner();
        }
        else if (commandLine.startsWith("list") || commandLine.startsWith("ls") || commandLine.startsWith("dir"))
        {
            String path = ".";
            if (commandLine.indexOf(' ') != -1)
                path = commandLine.substring(commandLine.indexOf(' ')).trim();
            printFileList(path);
        }
        else if (commandLine.startsWith("open"))
        {
            String path = "";
            if (commandLine.indexOf(' ') != -1)
                path = commandLine.substring(commandLine.indexOf(' ')).trim();
            if (path.trim().equals(""))
                System.err.println("You must specify a path in order to open a data file.");
            else
                printFileData(path);
        }
        else
        {
            System.err.println("Invalid command. Enter \"help\" to list possible commands.");
        }
    }
 
    /**
     * Prints opening banner.
     */
    public static void printBanner()
    {
        System.out.println("ITEC 441 Tilde Table Application by David Ball");
        System.out.println("");
        System.out.println(">> Enter \"help\" for usage information, enter \"exit\" to close.");
        System.out.println("");
        System.out.println("Enter a command at prompt:");
    }

    /**
     * Prints help page.
     */
    public static void printHelp()
    {
        System.out.println("ITEC 441 Tilde Table Application by David Ball");
        System.out.println("");
        System.out.println("About this application:");
        System.out.println("");
        System.out.println("Processes tilde-table files as specified in the ITEC 441 assignment details.");
        System.out.println("");
        System.out.println("Usage information:");
        System.out.println("");
        System.out.println("To use this application, type a command using the syntax below.");
        System.out.println("");
        System.out.println("  help         Prints this help page.");
        System.out.println("  list [path]  Lists all valid files to read at optional path.");
        System.out.println("  open [path]  Opens file at required path; relative paths are acceptible.");
        System.out.println("  version      Prints version information page.");
        System.out.println("");
        System.out.println("Enter a command:");
    }
    
    /**
     * Prints file list at path.
     */
    public static void printFileList(String path)
    {
        File directory = new File(path);
        try {
            System.out.println ("Browsing valid *.txt files in " 
                + directory.getAbsolutePath());
        } catch(SecurityException e) {
            System.out.println("CmdLineView SecurityException caught: " + e.getMessage());
            return;
        } catch(Exception e) {
            System.out.println("CmdLineView Exception caught: " + e.getMessage());
            return;
        }

        File[] listOfFiles = directory.listFiles();
        
        //short circuit on null list
        if (listOfFiles == null) return;
        
        //list dirs
        for (int d = 0; d < listOfFiles.length; d++) {
            if (listOfFiles[d].isDirectory()) {
                System.out.println("directory: " + listOfFiles[d].getName());
            }
        }
        
        //list files
        for (int f = 0; f < listOfFiles.length; f++) {
            if (listOfFiles[f].isFile() && listOfFiles[f].getName().toLowerCase().endsWith(".txt")) {
                System.out.println(".txt file: " + listOfFiles[f].getName());
            }
        }
    }
    
    /**
     * Opens file and prints the tilde table as rendered.
     */
    public static void printFileData(String path)
    {
        File file = new File(path);
        try {
            if (file.exists())
            {
                System.out.println ("Opening data file specified at " 
                    + file.getAbsolutePath() + ".");
            }
            else
            {
                System.err.println("File specified at " + path + " does not exist.");
                return;
            }
        } catch(SecurityException e) {
            System.out.println("CmdLineView SecurityException caught: " + e.getMessage());
            return;
        } catch(Exception e) {
            System.out.println("CmdLineView Exception caught: " + e.getMessage());
            return;
        }

        //get driver
        Driver driver = new TildeDriver();
        
        //connect to data
        if (!driver.open(path))
        {
            System.out.println("Failed to open file at path " + path + ".");
            return;
        }
        
        //get default relation
        Relation relation = driver.query("");
        
        //ask which columns and order the user wants in the report
        System.out.println("The following columns are in the tilde table:");
        for (int c = 0; c < relation.columnCount(); c++)
        {
            System.out.println("    " + (c+1) + ". " + relation.getColumn(c));
        }
        System.out.println("");
        System.out.println("Which columns do you want in the result set? List one per line, followed by Enter/Return.");
        System.out.println("Enter an empty line when finished.");
        System.out.println("");
        
        //get list of columns from user
        ArrayList<String> desiredColumns = new ArrayList<String>();
        String columnString = "";
        do
        {
            columnString = promptUser("Column " + (desiredColumns.size() + 1) + "? = ");

            int columnNumber = -1;
            try {
                //try by column number
                columnNumber = Integer.parseInt(columnString);
                //decrement by 1
                columnNumber--;
            }
            catch (NumberFormatException ex) {
                //if not a column number, then try the column name
                columnNumber = relation.getColumnIndex(columnString);
            }
            
            String columnName = "";
            try 
            {
                //convert column number into actual column name
                columnName = relation.getColumn(columnNumber);
                
                //store column name
                desiredColumns.add(columnName);
            
                //inform the user about the addition
                System.out.println("  -> Column " + (desiredColumns.size()) + " will be " + columnName + ".");
            }
            catch (ArrayIndexOutOfBoundsException ex)
            {
                if (columnString.length() > 0)
                {
                    //tell user about invalid
                    System.out.println("  !! Column \"" + columnString + "\" was not found.");
                    System.out.println("Please use a valid column name or number from the list above.");
                }
            }
        } while (columnString.length() > 0);
        
        //limit the projection
        try
        {
            relation.limitColumns(desiredColumns.toArray(new String[] {}));
        }
        catch (Exception e)
        {
            //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection limit can not be performed.");
        }
        
        System.out.println("");
        System.out.println("Table projected (via pipelining):");
        
        //list columns
        for (int c = 0; c < relation.columnCount(); c++)
        {
            System.out.print(String.format("%1$-15s", relation.getColumn(c)));
            if (relation.columnCount() != (c-1))
                //if not end of col list, print space
                System.out.print(" ");
        }
        System.out.println("");
        
        //list -------
        for (int c = 0; c < relation.columnCount(); c++)
        {
            System.out.print("---------------");
            if (relation.columnCount() != (c-1))
                //if not end of col list, print space
                System.out.print(" ");
        }
        System.out.println("");
        
        //run through rows
        Row row = relation.next();
        while (row != null)
        {
            for (int c = 0; c < row.columnCount(); c++)
            {
                System.out.print(String.format("%1$-15s", row.get(c)));
                if (row.columnCount() != (c-1))
                    //if not end of col list, print space
                    System.out.print(" ");
            }
            System.out.println("");
            //get next row
            row = relation.next();
        }
        
        //close data connection
        driver.close();
        
        //unset driver
        driver = null;
    }
    
    /**
     * Asks user for input. Returns command line entered.
     */
    public static String promptUser(String prompt)
    {
        System.out.print(prompt);
        Scanner scanner = new Scanner(System.in);
        return scanner.nextLine();
    }
}