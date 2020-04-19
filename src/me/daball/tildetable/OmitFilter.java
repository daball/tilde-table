package me.daball.tildetable;

/**
 * The OmitFilter omits rows in a Relation by a discriminating column/cell-value pair.
 * 
 * @author David Ball
 */
public class OmitFilter implements Filter
{
    private String[] columns;
    private String omittedColumn;
    private int omittedColumnAt = -1;
    private String[] omittedToValues;
    
    /**
     * Omits rows in a Relation on a column by cell values specified.
     */
    public OmitFilter(String[] columns, String column, String[] cellValues)
    {
        this.columns = columns;
        this.omittedColumn = column;
        this.omittedColumnAt = getColumnIndex(column);
        this.omittedToValues = cellValues;
    }
    
    /**
     * Copies the filter.
     */
    public OmitFilter clone()
    {
        OmitFilter copy = new OmitFilter(this.columns, this.omittedColumn, this.omittedToValues);
        return copy;
    }

    /**
     * Helper: Gets the column index at name.
     * 
     * @param  index   the column index to retrieve name
     * @return  Column index or -1 if not found
     */
    public int getColumnIndex(String name)
    {
        if (this.columns != null)
        {
            for (int i = 0; i < this.columns.length; i++)
            {
                if (this.columns[i].toUpperCase().equals(name.toUpperCase()))
                    return i;
            }
        }
        return -1;
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
     * Checks to see if this row will be valid by checking to make sure valid values are present.
     * Performs the check irrespective of uppercase/lowercase values. It also uses partial words
     * to make sure that similar matches make the cut.
     */
    public boolean isRowSkipped(Row row)
    {
        if (row == null) return false; //return false because other filters may want to filter
        for (String validValue: omittedToValues)
        {
            if (row.get(this.omittedColumnAt).toLowerCase().indexOf(validValue.toLowerCase()) >= 0)
                return true; //skip omitted values
        }
        return false; //allow anything else
    }
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        StringBuilder sb = new StringBuilder();
        sb.append("omit(\"" + this.omittedColumn + "\", new String[] {\n");
        for (String omittedValue: omittedToValues)
        {
            sb.append("                 \"" + omittedValue + "\",\n");
        }
        sb.delete(sb.length()-2,sb.length()); //remove last ,\n
        sb.append(" })");
        return sb.toString();
    }
}