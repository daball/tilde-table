package me.daball.tildetable;

/**
 * Counts every row into a new row number such that for every row each row will be uniquely
 * incremented.
 * 
 * @author David Ball
 */
public class CountFilter implements Filter
{
    private String[] columns;
    private String countColumnName;
    private int counter = 0;
    
    /**
     * Counts every row into a new row number such that for every row each row will be uniquely
     * incremented.
     */
    public CountFilter(String[] columns, String countColumnName)
    {
        this.columns = columns;
        this.countColumnName = countColumnName;
    }
    
    /**
     * Copies the current CountFilter.
     */
    public CountFilter clone()
    {
        CountFilter copy = new CountFilter(this.columns, this.countColumnName);
        copy.counter = this.counter;
        return copy;
    }
    
    /**
     * Gets the new column names provided when the count filter was created.
     */
    public String[] getColumns()
    {
        String[] newColumns = new String[this.columns.length+1];
        newColumns[0] = this.countColumnName;
        for (int c = 0; c < this.columns.length; c++)
            newColumns[c+1] = this.columns[c];
        return newColumns;
    }
    
    /**
     * Transforms the row data as it passes through the filter.
     */
    public Row transformRow(Row row)
    {
        //null pass-through
        if (row == null) return null;
        
        String[] cells = new String[this.columns.length+1];
        
        cells[0] = new Integer(++this.counter).toString();
        
        for (int c = 0; c < this.columns.length; c++)
        {
            try
            {
                cells[c+1] = row.get(c);
            }
            catch (Exception e)
            {
                cells[c+1] = ""; //use empty string as value
            }
        }
        
        //generate nev Row
        return new Row(cells);
    }
    
    /**
     * No rows are skipped in this filter. Always returns false. Resets counter when null comes around.
     */
    public boolean isRowSkipped(Row row)
    {
        if (row == null)
            this.counter = 0;
        return false;
    }
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        return "count(\"" + this.countColumnName + "\")";
    }

}