import java.util.ArrayList;

/**
 * Write a description of class FiltersView here.
 * 
 * @author David Ball
 * @version (a version number or a date)
 */
public class FilterView
{
    /**
     * Helper function to set up any and all filters before rendering.
     */
    public static Relation setupFilters(Relation relation)
    {
        //ask user to select a filter
        String answer = "";
        do
        {
            System.out.println("");
            System.out.println("The following filters are available:");
            System.out.println("    1. project  (Projects only the columns specified.)");
            System.out.println("    2. restrict (Restricts rows by a search criteria on a certain column.)");
            System.out.println("    3. omit     (Omits rows by a search criteria on a certain column.)");
            System.out.println("    4. distinct (Selects only distinct rows by indexing records at run-time.)");
            System.out.println("    5. sort     (Sorts rows on the columns specified.)");
            System.out.println("    6. reverse  (Reverses row order (filter materializes the view).)");
            System.out.println("    7. skip     (Skips an arbitrary number of rows.)");
            System.out.println("    8. limit    (Limits the number of rows returned.)");
            System.out.println("    9. count    (Counts the number of rows and inserts that value onto a new column.)");
            System.out.println("");
            answer = MainView.promptUser("Select a filter: ").trim().toLowerCase();
            if (answer.length() == 0)
            {
                return relation;
            }
            else if (answer.equals("1") || answer.equals("project"))
            {
                relation = setupProjectFilter(relation);
                return relation;
            }
            else if (answer.equals("2") || answer.equals("restrict"))
            {
                relation = setupRestrictFilter(relation);
                return relation;
            }
            else if (answer.equals("3") || answer.equals("omit"))
            {
                relation = setupOmitFilter(relation);
                return relation;
            }
            else if (answer.equals("4") || answer.equals("distinct"))
            {
                relation = setupDistinctFilter(relation);
                return relation;
            }
            else if (answer.equals("5") || answer.equals("sort"))
            {
                relation = setupSortFilter(relation);
                return relation;
            }
            else if (answer.equals("6") || answer.equals("reverse"))
            {
                relation = setupReverseFilter(relation);
                return relation;
            }
            else if (answer.equals("7") || answer.equals("skip"))
            {
                relation = setupSkipFilter(relation);
                return relation;
            }
            else if (answer.equals("8") || answer.equals("limit"))
            {
                relation = setupLimitFilter(relation);
                return relation;
            }
            else if (answer.equals("9") || answer.equals("count"))
            {
                relation = setupCountFilter(relation);
                return relation;
            }
            else
            {
                //otherwise display error
                System.out.println("Not a valid response.");
            }
        } while (answer.length() > 0);
        return relation;
    }
    
    /**
     * Helper function sets up a Project filter.
     */
    public static Relation setupProjectFilter(Relation relation)
    {
        //ask which columns and order the user wants in the report
        System.out.println("");
        System.out.println("You have selected Project filter.");
        System.out.println("");
        System.out.println("The following columns are in the projected table:");
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
            columnString = MainView.promptUser("Column " + (desiredColumns.size() + 1) + "? = ");

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
            relation = relation.project(desiredColumns.toArray(new String[] {}));
        }
        catch (Exception e)
        {
            //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection can not be performed.");
        }
        return relation;
    }
    
    /**
     * Helper function sets up a Restrict filter.
     */
    public static Relation setupRestrictFilter(Relation relation)
    {
        //ask which column the user wants
        System.out.println("");
        System.out.println("You have selected Restrict filter.");
        System.out.println("");
        System.out.println("The following columns are in the projected table:");
        for (int c = 0; c < relation.columnCount(); c++)
        {
            System.out.println("    " + (c+1) + ". " + relation.getColumn(c));
        }
        System.out.println("");
        System.out.println("Which column do you want to restrict in the result set?");
        System.out.println("");
        
        //get column name from user        
        String restrictedColumn = "";
        do
        {
            restrictedColumn = MainView.promptUser("Column? = ");

            int columnNumber = -1;
            try {
                //try by column number
                columnNumber = Integer.parseInt(restrictedColumn);
                //decrement by 1
                columnNumber--;
            }
            catch (NumberFormatException ex) {
                //if not a column number, then try the column name
                columnNumber = relation.getColumnIndex(restrictedColumn);
            }
            
            try 
            {
                //convert column number into actual column name
                restrictedColumn = relation.getColumn(columnNumber);
                
                //inform the user about the addition
                System.out.println("  -> Column " + restrictedColumn + " will be restricted in the results.");
                
                //is valid, break out of loop
                break;
            }
            catch (ArrayIndexOutOfBoundsException ex)
            {
                if (restrictedColumn.length() > 0)
                {
                    //tell user about invalid
                    System.out.println("  !! Column \"" + restrictedColumn + "\" was not found.");
                    System.out.println("Please use a valid column name or number from the list above.");
                }
            }
        } while (restrictedColumn.length() > 0);
        
        if (restrictedColumn.length() == 0) return relation;
        
        System.out.println("");
        System.out.println("Please list valid values for the selected column, one per line. Enter a blank line when done.");
        System.out.println("");
        
        //get list of valid values from user
        ArrayList<String> validValues = new ArrayList<String>();
        String valueString = "";
        do
        {
            valueString = MainView.promptUser("Valid value (" + (validValues.size()+1) + ") = ");
            if (valueString.length() > 0)
            {
                //add to valid values
                validValues.add(valueString);
                
                //inform the user about the addition
                System.out.println("  -> \"" + valueString + "\" will be valid for " + restrictedColumn + " in the results.");
            }
        } while (valueString.length() > 0);

        //restrict the projection
        try
        {
            relation = relation.restrict(restrictedColumn, validValues.toArray(new String[] { }));
        }
        catch (Exception e)
        {
            //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection restriction can not be performed.");
        }
        return relation;
    }
    
    /**
     * Helper function sets up a Omit filter.
     */
    public static Relation setupOmitFilter(Relation relation)
{
        //ask which column the user wants
        System.out.println("");
        System.out.println("You have selected Omit filter.");
        System.out.println("");
        System.out.println("The following columns are in the projected table:");
        for (int c = 0; c < relation.columnCount(); c++)
        {
            System.out.println("    " + (c+1) + ". " + relation.getColumn(c));
        }
        System.out.println("");
        System.out.println("Which column do you want to omit in the result set?");
        System.out.println("");
        
        //get column name from user        
        String omittedColumn = "";
        do
        {
            omittedColumn = MainView.promptUser("Column? = ");

            int columnNumber = -1;
            try {
                //try by column number
                columnNumber = Integer.parseInt(omittedColumn);
                //decrement by 1
                columnNumber--;
            }
            catch (NumberFormatException ex) {
                //if not a column number, then try the column name
                columnNumber = relation.getColumnIndex(omittedColumn);
            }
            
            try 
            {
                //convert column number into actual column name
                omittedColumn = relation.getColumn(columnNumber);
                
                //inform the user about the addition
                System.out.println("  -> Column " + omittedColumn + " will be omitted in the results.");
                
                //is valid, break out of loop
                break;
            }
            catch (ArrayIndexOutOfBoundsException ex)
            {
                if (omittedColumn.length() > 0)
                {
                    //tell user about invalid
                    System.out.println("  !! Column \"" + omittedColumn + "\" was not found.");
                    System.out.println("Please use a valid column name or number from the list above.");
                }
            }
        } while (omittedColumn.length() > 0);
        
        if (omittedColumn.length() == 0) return relation;
        
        System.out.println("");
        System.out.println("Please list values to inhibit from the selected column, one per line. Enter a blank line when done.");
        System.out.println("");
        
        //get list of valid values from user
        ArrayList<String> validValues = new ArrayList<String>();
        String valueString = "";
        do
        {
            valueString = MainView.promptUser("Valid value (" + (validValues.size()+1) + ") = ");
            if (valueString.length() > 0)
            {
                //add to valid values
                validValues.add(valueString);
                
                //inform the user about the addition
                System.out.println("  -> \"" + valueString + "\" will be blocked from s" + omittedColumn + " in the results.");
            }
        } while (valueString.length() > 0);

        //restrict the projection
        try
        {
            relation = relation.omit(omittedColumn, validValues.toArray(new String[] { }));
        }
        catch (Exception e)
        {
            //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection restriction can not be performed.");
        }
        return relation;
    }

    /**
     * Helper function sets up a Distinct filter.
     */
    public static Relation setupDistinctFilter(Relation relation)
    {
        //ask which column the user wants
        System.out.println("");
        System.out.println("You have selected Distinct filter.");
        System.out.println("");
        System.out.println("The following columns are in the projected table:");
        for (int c = 0; c < relation.columnCount(); c++)
        {
            System.out.println("    " + (c+1) + ". " + relation.getColumn(c));
        }
        System.out.println("");
        
        //ask if user wants to filter output
        String answer = "";
        do
        {
            System.out.println("Do you want to index the entire row? Choose No if you want to choose the index column. ");
            System.out.println("Choose Yes if the row needs to be unique across all cells. Blank line cancels the operation.");
            
            answer = MainView.promptUser("Use all columns in search index? [y,n] ").trim().toLowerCase();
            
            if (answer.trim().isEmpty()) return relation;
            
            if (answer.startsWith("n") || answer.length() == 0)
                break; //proceed with interview
            else if (answer.startsWith("y"))
            {
                //inform the user about the addition
                try {
                    relation = relation.distinct();
                    System.out.println("  -> All columns will be used to generate search tree so only distinct rows will be listed in results.");
                } catch (Exception e) {
                    System.out.println("An error occurred while setting up filter: " + e.getMessage());
                }
                return relation;
            }
            else
                //otherwise display error
                System.err.println("Not a valid response.");
        } while (answer.length() > 0);
        
        //ask which columns the user wants in the index
        System.out.println("");
        System.out.println("The following columns are in the projected table:");
        for (int c = 0; c < relation.columnCount(); c++)
        {
            System.out.println("    " + (c+1) + ". " + relation.getColumn(c));
        }
        System.out.println("");
        System.out.println("Which columns do you want to use as a search key? List one per line, followed by Enter/Return. Enter an empty line when finished.");
        System.out.println("");
        System.out.println("The search key will be used to discriminate whether or not a row is distinct among the result set.");
        
        //get list of columns from user
        ArrayList<String> desiredColumns = new ArrayList<String>();
        String columnString = "";
        do
        {
            columnString = MainView.promptUser("Key Column " + (desiredColumns.size() + 1) + "? = ");

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
                System.out.println("  -> Column " + (desiredColumns.size()) + ", " + columnName + ", will be used as a partial search key.");
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
            relation = relation.distinct(desiredColumns.toArray(new String[] {}));
        }
        catch (Exception e)
        {
            //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection distinction can not be performed.");
        }
        return relation;
    }
    
    /**
     * Helper function sets up a Sort filter.
     */
    public static Relation setupSortFilter(Relation relation)
    {
        //ask which column the user wants
        System.out.println("");
        System.out.println("You have selected Sort filter.");

        //ask which columns the user wants in the index
        System.out.println("");
        System.out.println("The following columns are in the projected table:");
        for (int c = 0; c < relation.columnCount(); c++)
        {
            System.out.println("    " + (c+1) + ". " + relation.getColumn(c));
        }
        System.out.println("");
        System.out.println("Which columns do you want to use as a search/sort key? List one per line, followed by Enter/Return. Enter an empty line when finished.");
        System.out.println("");
        System.out.println("The sort key will be used to discriminate whether or not a row is distinct among the result set. Nondistinct rows may not be in any order.");
        
        //get list of columns from user
        ArrayList<String> desiredColumns = new ArrayList<String>();
        String columnString = "";
        do
        {
            columnString = MainView.promptUser("Key Column " + (desiredColumns.size() + 1) + "? = ");

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
                System.out.println("  -> Column " + (desiredColumns.size()) + ", " + columnName + ", will be used as a partial sort key.");
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
        
        //sort the projection
        try
        {
            relation = relation.sort(desiredColumns.toArray(new String[] {}));
        }
        catch (Exception e)
        {
            //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection sort can not be performed.");
        }
        return relation;
    }
    
    /**
     * Helper function sets up a Reverse filter.
     */
    public static Relation setupReverseFilter(Relation relation)
    {
        //ask which column the user wants
        System.out.println("");
        System.out.println("You have selected Reverse filter.");
        System.out.println("");
        
        //ask if user wants to filter output
        String answer = "";
        do
        {
            System.out.println("Do you want to reverse the table direction? Choose No if you want to traverse the table forward. ");
            System.out.println("Choose Yes if you want to traverse the table backwards. Blank line cancels the operation.");

            System.out.println("Note that this will materialize the view in memory and it may take a while on large tables or be impossible for datasets larger than memory capacity.");
            
            answer = MainView.promptUser("Reverse table direction? [y,n] ").trim().toLowerCase();
            
            if (answer.trim().isEmpty()) return relation;
            
            if (answer.startsWith("n") || answer.length() == 0)
                break; //proceed with interview
            else if (answer.startsWith("y"))
            {
                //inform the user about the addition
                relation = relation.reverse();
                System.out.println("  -> All rows will be reversed.");
                return relation;
            }
            else
                //otherwise display error
                System.err.println("Not a valid response.");
        } while (answer.length() > 0);        
        return relation;
    }
    
    /**
     * Helper function sets up a Skip filter.
     */
    public static Relation setupSkipFilter(Relation relation)
    {
        System.out.println("");
        System.out.println("You have selected Skip filter.");
        System.out.println("");
        System.out.println("How many rows do you want to skip from the top of your results?");
        System.out.println("Put another way, which row number do you want to start with? (1-infinity)");
        System.out.println("Zero or blank line cancels the operation.");
        System.out.println("");
        
        //get column name from user        
        String rowsToSkip;
        int rowsToSkipInt;
        do
        {
            rowsToSkip = MainView.promptUser("Rows to skip? ");
            
            if (rowsToSkip.trim().isEmpty()) return relation;

            rowsToSkipInt = -1;
            try {
                //try by column number
                rowsToSkipInt = Integer.parseInt(rowsToSkip);
            }
            catch (NumberFormatException ex) {
                //tell user about invalid
                System.out.println("  !! \"" + rowsToSkip + "\" is not a number.");
            }
            
            if (rowsToSkipInt == 0)
            {
                //zero cancels
                return relation;
            }
            else if (rowsToSkipInt < 0)
            {
                //tell user about invalid
                System.out.println("  !! \"" + rowsToSkip + "\" is not an acceptable number. Can not skip a negative row. :~]");
            }            
            else
            {
                //otherwise valid, break out of loop
                break;
            }
        } while (rowsToSkip.length() > 0);
        
        //skip part of the projection
        try
        {
            if (rowsToSkipInt > 0)
                relation = relation.skip(rowsToSkipInt);
        }
        catch (Exception e)
        {
            //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection skip can not be performed.");
        }
        return relation;
    }
    
    /**
     * Helper function sets up a Limit filter.
     */
    public static Relation setupLimitFilter(Relation relation)
    {
        System.out.println("");
        System.out.println("You have selected Limit filter.");
        System.out.println("");
        System.out.println("How many rows do you want to include from the top of your results?");
        System.out.println("Put another way, where do you want to stop at? (1-infinity)");
        System.out.println("Zero or blank line cancels the operation.");
        System.out.println("");
        
        //get column name from user        
        String rowsToInclude;
        int rowsToIncludeInt;
        do
        {
            rowsToInclude = MainView.promptUser("Rows to include? ");

            if (rowsToInclude.trim().isEmpty()) return relation;
            
            rowsToIncludeInt = -1;
            try {
                //try by column number
                rowsToIncludeInt = Integer.parseInt(rowsToInclude);
            }
            catch (NumberFormatException ex) {
                //tell user about invalid
                System.out.println("  !! \"" + rowsToInclude + "\" is not a number.");
            }
            
            if (rowsToIncludeInt == 0)
            {
                //zero cancels
                return relation;
            }
            else if (rowsToIncludeInt < 0)
            {
                //tell user about invalid
                System.out.println("  !! \"" + rowsToInclude + "\" is not an acceptable number. Can not skip a negative row. :~]");
            }
            else
            {
                //otherwise valid, break out of loop
                break;
            }
            
        } while (rowsToInclude.length() > 0);
        
        //skip part of the projection
        try
        {
            if (rowsToIncludeInt > 0)
                relation = relation.limit(rowsToIncludeInt);
        }
        catch (Exception e)
        {
            //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection limit can not be performed.");
        }
        return relation;
    }    
    
    /**
     * Helper function sets up a Count filter.
     */
    public static Relation setupCountFilter(Relation relation)
    {
       System.out.println("");
       System.out.println("You have selected Count filter.");
       System.out.println("");
        
       //ask if user wants to filter output
       String answer = "";
       do
       {
           System.out.println("Do you want to count each row into a new column?");
           System.out.println("");
           
           answer = MainView.promptUser("Count rows into a new column? [y,n] ").trim().toLowerCase();
            
           if (answer.trim().isEmpty()) return relation;
            
           if (answer.startsWith("n") || answer.length() == 0)
               return relation; //abort
           else if (answer.startsWith("y"))
           {
               //break and proceed
               break;
           }
           else
               //otherwise display error
                System.err.println("Not a valid response.");
        } while (answer.length() > 0);
        
        String columnName = "";
        System.out.println("What do you want to call the new column? Blank line defaults to Id.");
        System.out.println("");
            
        columnName = MainView.promptUser("New Column Name? [Id] = ").trim();

        if (columnName.isEmpty())
            columnName = "Id";

       //inform the user about the addition
       System.out.println("  -> Column " + columnName + " will be used as a new counter column on the left.");
        
       try
       {
           return relation.count(columnName);
       }
       catch (Exception e)
       {
             //hopefully this will never happen if the code above is proper
            System.out.println("An unhandled exception has occurred:");
            System.out.println(e.getMessage());
            System.out.println("Projection count can not be performed.");
       }
       return relation;
   }   
}