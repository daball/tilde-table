package me.daball.tildetable;

/**
 * Inhibits row output until an arbitrary number of rows is skipped. Then it starts permits
 * normal row output.
 * 
 * @author David Ball
 */
public class SkipFilter implements Filter
{
    private String[] columns;
    private int rowsToSkip;
    private int rowsSkipped = 0; //counts rows skipped

    /**
     * Inhibits row output until an arbitrary number of rows is skipped. Then it starts permits
     * normal row output.
     */
    public SkipFilter(String[] columns, int rowsToSkip)
    {
        this.columns = columns;
        this.rowsToSkip = rowsToSkip;
    }

    /**
     * Copies the filter.
     */
    public SkipFilter clone()
    {
        SkipFilter copy = new SkipFilter(this.columns, this.rowsToSkip);
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
        if (rowsToSkip > rowsSkipped)
        {
            rowsSkipped++;
            if (row == null)
                //reset
                this.rowsSkipped = 0;
            return true;
        }
        else
        {
            if (row == null)
                //reset
                this.rowsSkipped = 0;
            return false;
        }
    }
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        return "skip(" + this.rowsToSkip + ")";
    }
}