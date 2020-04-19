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
 * enabled the consumer to 'limit' the projection by column names. (See the Relation.limit()
 * method for details.)
 * 
 * I think its a form of demand-driven pipelining, aka 'lazy' evaluation. The actual pipelining
 * takes place in the Relation.next() method.
 * 
 * The consumer may query the available columns using getColumn() and columnCount() prior to calling
 * limit() to ensure only valid column names are used to limit the projection.
 * 
 * @author David Ball
 */
public class Relation
{
    private Driver driver;
    private String[] actualColumns;
    private String[] columns;
    
    //this maps the projection, used to accelerate row selection algorithm
    private Integer[] projectionMap;

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
        this.columns = this.actualColumns = columns;
        //set projection map to put columns projected in the order that they appear
        this.projectionMap = new Integer[columns.length];
        for (int i = 0; i < columns.length; i++)
            this.projectionMap[i] = i;
    }

    /**
     * Gets the projected column name at index.
     * 
     * @param  index   the column index to retrieve name
     * @return  String with column name
     */
    public String getColumn(int index)
    {
        if (this.columns == null)
            return null;
        else
            return this.columns[index];
    }
    
    /**
     * Gets the actual column name at index (before the projection is limited).
     * 
     * @param  index   the column index to retrieve name
     * @return  String with column name
     */
    public String getActualColumn(int index)
    {
        if (this.actualColumns == null)
            return null;
        else
            return this.actualColumns[index];
    }
    
    /**
     * Gets the projected column index at name.
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
     * Gets the actual column index at name (before the projection is limited).
     * 
     * @param  index   the column index to retrieve name
     * @return  Column index or -1 if not found
     */
    public int getActualColumnIndex(String name)
    {
        if (this.actualColumns != null)
        {
            for (int i = 0; i < this.actualColumnCount(); i++)
            {
                if (this.getActualColumn(i).toUpperCase().equals(name.toUpperCase()))
                    return i;
            }
        }
        return -1;
    }

    /**
     * Returns the projected number of columns.
     */
    public int columnCount()
    {
        if (this.columns != null)
            return this.columns.length;
        else
            return 0;
    }
    
    /**
     * Returns the actual number of columns (before the projection is limited).
     */
    public int actualColumnCount()
    {
        if (this.actualColumns != null)
            return this.actualColumns.length;
        else
            return 0;
    }
    

    /**
     * Limits the columns (and column order) in the projection.
     */
    public void limitColumns(String[] columns) throws Exception
    {
        Integer proposedProjectionMap[] = new Integer[columns.length];
        for (int c = 0; c < columns.length; c++)
        {
            String column = columns[c];
            //map projection to actual columns needed in result set.
            int actualColumnAt = this.getActualColumnIndex(column);
            if (actualColumnAt == -1)
                throw new Exception("Requested column \"" + column + "\" is not a valid column.");
            else
                proposedProjectionMap[c] = actualColumnAt;
        }
        //as long as we pass without error, update the instance data
        this.projectionMap = proposedProjectionMap;
        this.columns = columns;
    }
    
    /**
     * Gets the next Row.
     * 
     * @return  Row or null when no more rows.
     */
    public Row next()
    {
        //get unprocessed row from driver
        Row actualRow = driver.nextRow();
        
        //if at end of set, stop processing and echo that result
        if (actualRow == null) return null;

        //hold projected cells
        String[] cells = new String[this.columnCount()];
        
        //remap the cells based on limit (and order) from limit()
        for (int c = 0; c < this.columnCount(); c++)
        {
            cells[c] = actualRow.get(this.projectionMap[c]);
        }
        
        //generate and return a new Row
        Row row = new Row(cells);
        return row;
    }
}