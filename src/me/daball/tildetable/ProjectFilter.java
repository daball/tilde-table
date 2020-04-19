package me.daball.tildetable;

/**
 * The ProjectFilter limits or arranges columns in a Relation so that the output
 * Rows in the Relation are limited or rearranged from their inputs.
 * 
 * @author David Ball
 */
public class ProjectFilter implements Filter
{
    private String[] actualColumns;
    private String[] columns;
    
    //this maps the projection, used to accelerate row selection algorithm
    private Integer[] projectionMap;

    /**
     * Projects only the columns specified in newColumns.
     */
    public ProjectFilter(String[] actualColumns, String[] newColumns) throws Exception
    {
        this.actualColumns = actualColumns;
        this.columns = newColumns;
        //create projection map to speed up traversal query across large data sets with unneeded logic
        Integer proposedProjectionMap[] = new Integer[this.columns.length];
        for (int c = 0; c < this.columns.length; c++)
        {
            String column = this.columns[c];
            //map projection to actual columns needed in result set.
            int actualColumnAt = this.getActualColumnIndex(column);
            if (actualColumnAt == -1)
                throw new Exception("Requested column \"" + column + "\" is not a valid column.");
            else
                proposedProjectionMap[c] = actualColumnAt;
        }
        //as long as we pass without error, update the instance data
        this.projectionMap = proposedProjectionMap;
    }

    /**
     * Copies the filter.
     */
    public ProjectFilter clone()
    {
        try
        {
            ProjectFilter copy = new ProjectFilter(this.actualColumns, this.columns);
            return copy;
        }
        catch (Exception e)
        {
            return null;
        }
    }
    
    /**
     * Helper: Gets the actual column index at name (before the projection is limited).
     * 
     * @param  index   the column index to retrieve name
     * @return  Column index or -1 if not found
     */
    protected int getActualColumnIndex(String name)
    {
        if (this.actualColumns != null)
        {
            for (int i = 0; i < this.actualColumns.length; i++)
            {
                if (this.actualColumns[i].toUpperCase().equals(name.toUpperCase()))
                    return i;
            }
        }
        return -1;
    }
    
    /**
     * Gets the new column names provided when the project filter was created.
     */
    public String[] getColumns()
    {
        return this.columns;
    }
    
    /**
     * Transforms the row data as it passes through the filter.
     */
    public Row transformRow(Row row)
    {
        //if row is null, return null
        if (row == null) return null;
        
        //hold projected cells
        String[] cells = new String[this.columns.length];
        
        //remap the cells based on limit (and order) from projection map
        for (int c = 0; c < this.columns.length; c++)
        {
            try
            {
                cells[c] = row.get(this.projectionMap[c]);
            }
            catch (Exception e)
            {
                cells[c] = ""; //use empty string as value
            }
        }
        
        //generate and return a new Row
        return new Row(cells);
    }
    
    /**
     * No rows are skipped in this filter. Always returns false.
     */
    public boolean isRowSkipped(Row row)
    {
        return false;
    }
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        StringBuilder sb = new StringBuilder();
        sb.append("project(new String[] {\n");
        for (String column: columns)
        {
            sb.append("                 \"" + column + "\",\n");
        }
        sb.delete(sb.length()-2,sb.length()); //remove last ,\n
        sb.append(" })");
        return sb.toString();
    }
}