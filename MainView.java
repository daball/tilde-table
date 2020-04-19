import java.util.Scanner;

/**
 * MainView creates the command-line prompt view for the application and dispatches
 * program logic to more specific command-line views.
 * 
 * To initiate the view logic, call runLoop().
 * 
 * @author David Ball
 */
public class MainView
{
    /**
     * Runs the sample specified in the assignment.
     * To call it run app with --sample or issue sample command at main prompt.
     */
    public static void runSample()
    {
        //print hello
        MainView.printBanner();
        //load file
        RelationView.loadFileToRelation("sample-cars.txt");
        //get relation
        Relation relation = Controller.relations.get("SAMPLE-CARS");
        //describe relation
        RelationView.describeRelation("SAMPLE-CARS");
        //print relation
        RelationView.selectRelation("SAMPLE-CARS");
        //filter relation per sample
        String[] desiredColumns = {"Make","Model","Price"};
        Relation projection;
        try
        {
            projection = relation.project(desiredColumns);
        }
        catch (Exception e)
        {
            projection = relation;
            System.out.println("An error occurred during projection: " + e.getMessage());
        }
        //store projection
        Controller.relations.put("SAMPLE-CARS-WITH-PROJECTION", projection);
        //describe filtered relation
        RelationView.describeRelation("SAMPLE-CARS-WITH-PROJECTION");
        //print filtered relation
        RelationView.selectRelation("SAMPLE-CARS-WITH-PROJECTION");
    }
    
    /**
     * Runs the main program view loop. User must type exit to break out.
     * 
     * @param  args   any command-line args
     * @return     0, no errors
     */
    public static void runLoop(String[] args)
    {
        //print help page
        printHelp();
        //loop on the command-line
        String commandLine = "";
        while (!commandLine.startsWith("exit") && !commandLine.startsWith("quit"))
        {
            commandLine = promptUser("itec441-david-ttms# ").trim();
            if (commandLine != "")
            {
                dispatchCommandLine(commandLine);
            }
        }
        //print exit message
        System.out.println("Goodbye!");
    }
    
    /**
     * Dispatches/routes any command provided to the proper command handler.
     */
    private static void dispatchCommandLine(String commandLine)
    {
        if (commandLine == null || commandLine.trim().isEmpty())
        {
            //short circuit, no command entered, nothing to dispatch
            return;
        }
        else if (commandLine.toLowerCase().startsWith("exit") || commandLine.toLowerCase().startsWith("quit"))
        {
            //short circuit, nothing to dispatch, but this should break any loop pending an exit
            return;
        }
        else if (commandLine.toLowerCase().startsWith("help") || commandLine.toLowerCase().startsWith("?"))
        {
            printHelp();
        }
        else if (commandLine.toLowerCase().startsWith("exit") || commandLine.toLowerCase().startsWith("quit"))
        {
            //run sample
            runSample();
        }
        else if (commandLine.toLowerCase().startsWith("ver"))
        {
            printBanner();
        }
        else if (commandLine.toLowerCase().startsWith("list") || commandLine.toLowerCase().startsWith("ls") || commandLine.toLowerCase().startsWith("dir"))
        {
            String path = ".";
            if (commandLine.indexOf(' ') != -1)
                path = commandLine.substring(commandLine.indexOf(' ')).trim();
            RelationView.printFileList(path);
        }
        else if (commandLine.toLowerCase().startsWith("open") || commandLine.toLowerCase().startsWith("load"))
        {
            String path = "";
            if (commandLine.indexOf(' ') != -1)
                path = commandLine.substring(commandLine.indexOf(' ')).trim();
            if (path.trim().equals(""))
                System.err.println("You must specify a path in order to open a data file.");
            else
                RelationView.loadFileToRelation(path);
        }
        else if (commandLine.toLowerCase().startsWith("show"))
        {
            RelationView.showRelations();
        }
        else if (commandLine.toLowerCase().startsWith("sel") || commandLine.toLowerCase().startsWith("proj"))
        {
            String relation = "";
            if (commandLine.indexOf(' ') != -1)
                relation = commandLine.substring(commandLine.indexOf(' ')).trim();
            if (relation.trim().equals(""))
                System.err.println("You must specify a relation in order to project the table.");
            else
                RelationView.selectRelation(relation);
        }
        else if (commandLine.toLowerCase().startsWith("desc"))
        {
            String relation = "";
            if (commandLine.indexOf(' ') != -1)
                relation = commandLine.substring(commandLine.indexOf(' ')).trim();
            if (relation.trim().equals(""))
                System.err.println("You must specify a relation in order to close the table.");
            else
                RelationView.describeRelation(relation);
        }
        else if (commandLine.toLowerCase().startsWith("filter"))
        {
            String relation = "";
            if (commandLine.indexOf(' ') != -1)
                relation = commandLine.substring(commandLine.indexOf(' ')).trim();
            if (relation.trim().equals(""))
                System.err.println("You must specify a relation in order to filter the table.");
            else
                RelationView.filterRelation(relation);
        }
        else if (commandLine.toLowerCase().startsWith("close") || commandLine.toLowerCase().startsWith("unload"))
        {
            String relation = "";
            if (commandLine.indexOf(' ') != -1)
                relation = commandLine.substring(commandLine.indexOf(' ')).trim();
            if (relation.trim().equals(""))
                System.err.println("You must specify a relation in order to close the table.");
            else
                RelationView.unloadRelation(relation);
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
    }

    /**
     * Prints help page.
     */
    public static void printHelp()
    {
        printBanner();
        System.out.println("About this application:");
        System.out.println("  Processes tilde-table files as specified in the ITEC 441 assignment details.");
        System.out.println("  To use this application, type a command using the syntax below.");
        System.out.println("");
        System.out.println("Basic commands:");
        System.out.println("  exit|quit      Exits/quits the application.");
        System.out.println("  ?|help         Prints this help page.");
        System.out.println("  sample         Runs the sample routine.");
        System.out.println("  ver[sion]      Prints version information page.");
        System.out.println("");
        System.out.println("Relation commands (in order of sequence):");
        System.out.println("  ls [path]      Lists all valid (*.txt) files to read at optional path.");
        System.out.println("  open [path]    Opens file at *required* path to a named relation.");
        System.out.println("  show           Lists all loaded relations by name.");
        System.out.println("  describe [rel] Describes a relation. Includes name, path, and filters.");
        //System.out.println("  rename [rel]   Renames a relation provided a valid relation name.");
        System.out.println("  filter [rel]   Applies filters on a relation as a seperate relation.");
        //System.out.println("  unfilter [rel] Removes filters from a relation.");
        System.out.println("  project [rel]  Prints the contents of the relation.");
        System.out.println("  close [rel]    Unloads a relation.");
        System.out.println("");
        System.out.println("Enter a command:");
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