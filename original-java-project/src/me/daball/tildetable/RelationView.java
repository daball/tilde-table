package me.daball.tildetable;

import java.io.File;
import java.util.Date;

/**
 * RelationView holds view logic for managing Relation objects.
 * 
 * @author David Ball
 */
public class RelationView
{
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
                System.out.println("    [Directory] " + listOfFiles[d].getName());
            }
        }
        
        //list files
        for (int f = 0; f < listOfFiles.length; f++) {
            if (listOfFiles[f].isFile() && listOfFiles[f].getName().toLowerCase().endsWith(".txt")) {
                System.out.println("    [Text file] " + listOfFiles[f].getName());
            }
        }
    }
    
    /**
     * Loads file and prints the tilde table as rendered.
     */
    public static void loadFileToRelation(String path)
    {
        File file = new File(path);
        try {
            if (file.exists())
            {
                System.out.println ("Loading data file specified at " 
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
            System.out.println("Failed to load file at path " + path + ".");
            return;
        }
        
        //get default relation
        Relation relation = driver.query("");
        
        //generate relation name, same as file name, but without file extension, spaces become underscores
        String relationName = file.getName().toUpperCase().replace(' ', '_');
        if (relationName.indexOf(".") != -1)
        {
            relationName = relationName.substring(0, relationName.lastIndexOf("."));
        }
        
        //store relation in the global relations tree as file name (without the extension)
        Controller.relations.put(relationName, relation);
        
        System.out.println("Relation " + relationName + " stored.");
    }
        
    /**
     * Filters a relation into another relation.
     */
    public static void filterRelation(String relationName)
    {
        Relation relation = Controller.getRelation(relationName);
        if (relation != null)
            System.out.println ("Filtering relation " 
                + relationName.toUpperCase() + ".");
        else
        {
            System.err.println("Relation specified \"" + relationName + "\" does not exist.");
            return;
        }
    
        //ask if user wants to filter output
        String answer = "";
        do
        {
            //ask which column the user wants
            System.out.println("");
            System.out.println("The following columns are in the projected table:");
            for (int c = 0; c < relation.columnCount(); c++)
            {
                System.out.println("    " + (c+1) + ". " + relation.getColumn(c));
            }
            System.out.println("");
            System.out.println("Do you want to further filter the projected output?");
            answer = MainView.promptUser("Filter? [y,n] ").trim().toLowerCase();
            if (answer.startsWith("n") || answer.length() == 0)
                break;
            else if (answer.startsWith("y"))
                relation = FilterView.setupFilters(relation);
            else
                //otherwise display error
                System.err.println("Not a valid response.");
        } while (answer.length() > 0);

        System.out.println("What name do you want to call the new relation?");
        String newRelationName = MainView.promptUser("Relation name? = ").toUpperCase().replace(' ', '_');
        if (newRelationName.isEmpty())
            newRelationName = relationName.toUpperCase().replace(' ', '_');
        
        //inform the user about the addition
        System.out.println("  -> " + newRelationName + " will be used for the filtered relation.");
        
        Controller.relations.put(newRelationName, relation);
    }
    
    /**
     * Renders relation output.
     */
    public static void selectRelation(String relationName)
    {
        Relation relation = Controller.getRelation(relationName);
        if (relation == null)
        {
            System.err.println("Relation specified \"" + relationName + "\" does not exist.");
            return;
        }
        System.out.println(relationName.toUpperCase());
        for (int i = 0; i < relationName.length(); i++)
            System.out.print("=");
        System.out.println("");
        
        //starting timestamp
        long timestamp1, timestamp2;
        timestamp1 = new Date().getTime();
        
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
        System.out.println(relation.rowsReturned() + " rows returned.");

        timestamp2 = new Date().getTime();
                
        //print traversal metric
        System.out.println("Traversed table, filtered projection, and rendered output in " + (timestamp2-timestamp1) + " ms.");
        
        //rewind the relation so it can be re-read
        relation.rewind();
    }

    /**
     * Lists all the loaded or generated relations.
     */
    public static void showRelations()
    {
        System.out.println("Listing named relations:");
    
        //list relations
        for (String relationName: Controller.relations.keySet()) {
            System.out.println("  " + relationName);
        }
        
        System.out.println("Total relations: " + Controller.relations.size());
    }
    
    /**
     * Describes the relation specified.
     */
    public static void describeRelation(String relationName)
    {
       relationName = relationName.toUpperCase();
       Relation relation = Controller.getRelation(relationName);
       if (relation == null)
       {
           System.err.println("Relation specified " + relationName + " does not exist.");
           return;
       }
       System.out.println("Relation description:");
       System.out.println("  NAME=" 
           + relationName);
       if (relation.getFilters().size() > 0)
       {
            System.out.println("Filters applied to relation:");
            System.out.println("  relation");
            for (Filter filter: relation.getFilters())
            {
                String[] descLines = filter.toString().split("\n");
                
                StringBuilder desc = new StringBuilder();
                desc.append("    .");
                for (String descLine: descLines)
                {
                    desc.append(descLine);
                    desc.append("\n     ");
                }
                System.out.println("    " + desc.toString().trim());
            }
       }
       System.out.println("Relation contains column fields:");
       for (int c = 0; c < relation.columnCount(); c++)
       {
           System.out.println("    " + (c+1) + ". " + relation.getColumn(c));
       }
    }
    
    /**
     * Closes the relation specified.
     */
    public static void unloadRelation(String relationName)
    {
       relationName = relationName.toUpperCase();
       Relation relation = Controller.getRelation(relationName);
       if (relation != null)
           System.out.println ("Unloading relation \"" 
               + relationName + "\".");
       else
       {
           System.err.println("Relation specified \"" + relationName + "\" does not exist.");
           return;
       }
       
       //close data connection
       //relation.close(); //breaks alternate views, let the garbage collector handle it
       
       //remove the relation
       Controller.relations.remove(relationName);
    }
}