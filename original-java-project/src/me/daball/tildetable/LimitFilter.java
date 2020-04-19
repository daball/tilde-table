package me.daball.tildetable;

/**
 * Processes row output until an arbitrary number of rows is returned. Then it stops permitting
 * normal row output and inhibits processing.
 * 
 * @author David Ball
 */
public class LimitFilter implements Filter
{
    private String[] columns;
    private int rowsPermitted;
    private int rowsReturned = 0; //counts rows skipped

    /**
     * Processes row output until an arbitrary number of rows is returned. Then it stops permitting
     * normal row output and inhibits processing.
     */
    public LimitFilter(String[] columns, int rowsPermitted)
    {
        this.columns = columns;
        this.rowsPermitted = rowsPermitted;
    }

    /**
     * Copies the filter.
     */
    public LimitFilter clone()
    {
        LimitFilter copy = new LimitFilter(this.columns, this.rowsPermitted);
        return copy;
    }
    
    /**
     * Gets the column names provided when the filter was initialized.
     */
    public String[] getColumns()
    {
        return this.columns;
    }
    
    /**
     * Returns the row back out as is.
     */
    public Row transformRow(Row row)
    {
        //return the input Row
        return row;
    }
    
    /**
     * Checks to see if this row has already been used will be valid.
     */
    public boolean isRowSkipped(Row row)
    {
        //check for null
        if (row == null)
        {
            //reset for next run
            rowsReturned = 0;
            return false; //don't skip it, definitely pass it along because of infinite looping
        }
        if (rowsReturned < rowsPermitted)
        {
            //row is permitted
            rowsReturned++; //increment counter
            return false;
        }
        else
            //row is inhibited
            return true;
    }
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        return "limit(" + this.rowsPermitted + ")";
    }
}