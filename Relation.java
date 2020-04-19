import java.util.ArrayList;

/**
 * Relation is created for data queries.
 * 
 * Note:
 * 
 * I have chosen not to materialize the data. I have sit and thought this problem through
 * and I just don't think that materializing the data is the best choice. I do understand
 * how I would have to process the Relation if a materialization scheme is used.
 * 
 * The cost of writing the entire table to disk prior to generating a single set of results
 * is expensive in terms of time. It actually (at this point) seems to me to be much easier
 * to actually pipeline the stream instead of rewrite this table to disk and then reprocess
 * the new table.
 * 
 * That said, I'm not sure if the following solution represents a true pipeline. I have simply
 * enabled the consumer to limit the projection by column names. (See the Relation.project()
 * method for details.)
 * 
 * I think its a form of demand-driven pipelining, aka 'lazy' evaluation. The actual pipelining
 * takes place in the Relation.next() method.
 * 
 * The consumer may query the available columns using getColumn() and columnCount() prior to calling
 * project() to ensure only valid column names are used to limit the projection.
 * 
 * The Relation supports a number of other possible filters, though nothing really comes to mind
 * at the moment. Perhaps some type of foreign key filter could be realized as well.
 * 
 * @author David Ball
 */
public class Relation implements Cloneable
{
    private Driver driver;
    private String[] columns;
    
    //this maps the projection, used to accelerate row selection algorithm
    private ArrayList<Filter> filters = new ArrayList<Filter>();
    
    //counter
    private int countRowsReturned = 0;

    /**
     * Creates a relation with list of columns and current driver.
     * Used internally by model/Driver. Don't use this in consumer.
     * 
     * @param  driver   the driver hosting the Relation
     * @param  columns  an array of columns to become the default Relation projection
     */
    Relation(Driver driver, String[] columns)
    {
        this.driver = driver;
        this.columns = columns;
    }
    
    /**
     * Copies the relation to a new object.
     */
    public Relation clone()
    {
        Relation copy = new Relation(this.driver, this.columns);
        copy.filters = new ArrayList<Filter>();
        for (Filter f: this.filters)
        {
            copy.filters.add(f.clone());
        }
        copy.countRowsReturned = this.countRowsReturned;
        return copy;
    }

    /**
     * Gets the projected/filtered columns.
     * 
     * @return  String with column name
     */
    public String[] getColumns()
    {
        if (this.columns == null)
            return null;
        else
            //check for filters
            if (this.filters.size() == 0)
                //if none, use initial layout
                return this.columns;
            else
                //otherwise ask the last filter in the chain what's up
                return this.filters.get(this.filters.size()-1).getColumns();
    }
    
    /**
     * Gets the projected/filtered column name at index.
     * 
     * @param  index   the column index to retrieve name
     * @return  String with column name
     */
    public String getColumn(int index)
    {
        if (this.columns == null)
            return null;
        else
            return this.getColumns()[index];
    }
        
    /**
     * Gets the projected/filtered column index at name.
     * 
     * @param  index   the column index to retrieve name
     * @return  Column index or -1 if not found
     */
    public int getColumnIndex(String name)
    {
        if (this.columns != null)
        {
            for (int i = 0; i < this.columnCount(); i++)
            {
                if (this.getColumn(i).toUpperCase().equals(name.toUpperCase()))
                    return i;
            }
        }
        return -1;
    }
    
    /**
     * Returns the projected/filtered number of columns.
     */
    public int columnCount()
    {
        if (this.columns != null)
            return this.getColumns().length;
        else
            return 0;
    }
        
    /**
     * Gets the list of filters.
     */
    public ArrayList<Filter> getFilters()
    {
        return this.filters;
    }
    
    /**
     * Limits the columns (and column order) in the projection.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation project(String[] columns) throws Exception
    {
        //create filter
        ProjectFilter filter = new ProjectFilter(this.getColumns(), columns);
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Restricts row output on a column where cell values are valid.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation restrict(String column, String[] validValues) throws Exception
    {
        //create filter
        RestrictFilter filter = new RestrictFilter(this.getColumns(), column, validValues);
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Omits row output on a column where cell values are valid.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation omit(String column, String[] blockValues) throws Exception
    {
        //create filter
        OmitFilter filter = new OmitFilter(this.getColumns(), column, blockValues);
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Overloaded: Restricts row output to distinct rows. Uses whole row for search index.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation distinct() throws Exception
    {
        //create filter
        DistinctFilter filter = new DistinctFilter(this.getColumns());
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Overloaded: Restricts row output to distinct rows. Provide search columns to use as index key.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation distinct(String[] columns) throws Exception
    {
        //create filter
        DistinctFilter filter = new DistinctFilter(this.getColumns(), columns);
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Sorts row output by sort columns used as index key. Materializes view in memory, be sure
     * to limit projection prior to sorting.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation sort(String[] columns) throws Exception
    {
        //create filter
        SortFilter filter = new SortFilter(this.getColumns(), columns);
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Reverses row output. Materializes view in memory, be sure to limit projection prior to sorting.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation reverse()
    {
        //create filter
        ReverseFilter filter = new ReverseFilter(this.getColumns());
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Skips the number of rows specified.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation skip(int rowsToSkip) throws Exception
    {
        //create filter
        SkipFilter filter = new SkipFilter(this.getColumns(), rowsToSkip);
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Allows the number of rows specified.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation limit(int rowsToAllow) throws Exception
    {
        //create filter
        LimitFilter filter = new LimitFilter(this.getColumns(), rowsToAllow);
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Counts output rows into a column as specified.
     * 
     * @return  The Relation object so daisy-chaining operations is easier to code.
     */
    public Relation count(String countColumn) throws Exception
    {
        //create filter
        CountFilter filter = new CountFilter(this.getColumns(), countColumn);
        //add filter to filter list
        Relation copy = this.clone();
        copy.filters.add(filter);
        return copy;
    }
    
    /**
     * Gets the next Row.
     * 
     * @return  Row or null when no more rows.
     */
    public Row next()
    {
        boolean isRowSkipped;
        do
        {            
            //start by not skipping the row
            isRowSkipped = false;
            
            //get unprocessed row from driver
            Row row = driver.nextRow();
        
            //run the row through any and all filters to first make sure we're not skipping it
            for (int f = 0; f < this.filters.size(); f++)
            {
                row = this.filters.get(f).transformRow(row);
                isRowSkipped = this.filters.get(f).isRowSkipped(row);
                if (isRowSkipped) break; //if row is skipped, break out of for loop and try the next one
            }

            if (!isRowSkipped) //if row is not skipped
            {
                if (row != null)
                //{
                    //increment counter
                    this.countRowsReturned++;
                
                //return valid row
                return row;
            }
        } while (true); //loop forever ;-), or until null, or until valid row breaks out of loop
    }
    
    /**
     * Gets the number of rows returned.
     * 
     * @return  The number of rows returned so far.
     */
    public int rowsReturned()
    {
        return this.countRowsReturned;
    }
    
    /**
     * Rewinds back to the beginning of the data set.
     */
    public void rewind()
    {
        this.driver.rewind();
        this.countRowsReturned = 0;
    }
    
    public void close()
    {
        this.driver.close();
    }
}