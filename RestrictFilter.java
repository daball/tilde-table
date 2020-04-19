/**
 * The RestrictFilter restricts rows in a Relation by a discriminating column/cell-value pair.
 * 
 * @author David Ball
 */
public class RestrictFilter implements Filter
{
    private String[] columns;
    private String restrictedColumn;
    private int restrictedColumnAt = -1;
    private String[] restrictedToValues;
    
    /**
     * Restricts rows in a Relation on a column by cell values specified.
     */
    public RestrictFilter(String[] columns, String column, String[] cellValues)
    {
        this.columns = columns;
        this.restrictedColumn = column;
        this.restrictedColumnAt = getColumnIndex(column);
        this.restrictedToValues = cellValues;
    }
    
    /**
     * Copies the filter.
     */
    public RestrictFilter clone()
    {
        RestrictFilter copy = new RestrictFilter(this.columns, this.restrictedColumn, this.restrictedToValues);
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
        for (String validValue: restrictedToValues)
        {
            if (row.get(this.restrictedColumnAt).toLowerCase().indexOf(validValue.toLowerCase()) >= 0)
                return false; //permit valid value
        }
        return true; //skip anything not listed
    }
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        StringBuilder sb = new StringBuilder();
        sb.append("restrict(\"" + this.restrictedColumn + "\", new String[] {\n");
        for (String restrictedValue: restrictedToValues)
        {
            sb.append("                 \"" + restrictedValue + "\",\n");
        }
        sb.delete(sb.length()-2,sb.length()); //remove last ,\n
        sb.append(" })");
        return sb.toString();
    }
}