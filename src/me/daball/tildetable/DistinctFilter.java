package me.daball.tildetable;

import java.util.TreeMap;

/**
 * DistinctFilter ensures that a row is distinct on a column (or on the row as a whole) depending
 * on its initialization.
 * 
 * @author David Ball
 */
public class DistinctFilter implements Filter
{
    private TreeMap<String,String> searchTree = new TreeMap<String,String>();
    private String[] actualColumns;
    private String[] indexedColumns;

    //this maps the indexed columns to the actual columns, used to accelerate row selection algorithm during search query
    private Integer[] indexMap;
    
    /**
     * Creates a DistinctFilter that makes sure each row (as a whole) is unique. All cells are tested
     * against all cells in all rows.
     */
    public DistinctFilter(String[] columns)
    {
        this.actualColumns = this.indexedColumns = columns;
        this.indexMap = new Integer[this.indexedColumns.length];
        //since we're using the whole row as an index and both the actual and the indexed columns are
        //identical, we're going to default our index map to reflect this
        for (int c = 0; c < this.indexedColumns.length; c++)
            this.indexMap[c] = c;
    }
    
    /**
     * Creates a DistinctFilter that makes sure each row (as a whole) is unique. All indexed column
     * cells are tested against prior indexed cells from all rows before them.
     */
    public DistinctFilter(String[] columns, String[] indexedColumns) throws Exception
    {
        this.actualColumns = columns;
        this.indexedColumns = indexedColumns;
        //create index map to speed up traversal query across large data sets with unneeded logic
        Integer proposedIndexMap[] = new Integer[this.indexedColumns.length];
        for (int c = 0; c < this.indexedColumns.length; c++)
        {
            String column = this.indexedColumns[c];
            //map projection to actual columns needed in result set.
            int actualColumnAt = this.getActualColumnIndex(column);
            if (actualColumnAt == -1)
                throw new Exception("Requested column \"" + column + "\" is not a valid column.");
            else
                proposedIndexMap[c] = actualColumnAt;
        }
        //as long as we pass without error, update the instance data
        this.indexMap = proposedIndexMap;
    }
    
    /**
     * Copies the filter.
     */
    public DistinctFilter clone()
    {
        try
        {
            DistinctFilter copy = new DistinctFilter(this.actualColumns, this.indexedColumns);
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
     * Returns a string with each cell seperated by tildes.
     */
    public String generateRowKey(Row row)
    {
        StringBuilder sb = new StringBuilder();
        //remap the cells based on limit (and order) from projection map
        for (int c = 0; c < this.indexedColumns.length; c++)
        {
            //try
            {
                sb.append(row.get(this.indexMap[c]));
                sb.append("~");
            }
            //catch (Exception e)
            {
            }
        }
        if (sb.length()>0)
            sb.deleteCharAt(sb.length()-1);
        //System.out.println("GENROWKEY="+sb.toString());
        return sb.toString();
    }
    
    /**
     * Gets the column names provided when the filter was initialized.
     */
    public String[] getColumns()
    {
        return this.actualColumns;
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
        boolean hasKey = false;
        //check for null
        if (row == null)
        {
            //once null is reached, reset
            searchTree.clear();
            return false; //don't skip it, not sure if it matters, but definitely pass it along
        }
        else
            hasKey = searchTree.containsKey(generateRowKey(row)); //skip it if you do, use it if you don't
        if (hasKey)
            return true;
        //store row's key
        searchTree.put(generateRowKey(row), row.toString());
        return false;
    }

    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        StringBuilder sb = new StringBuilder();
        sb.append("distinct(new String[] {\n");
        for (String column: indexedColumns)
        {
            sb.append("                 \"" + column + "\",\n");
        }
        sb.delete(sb.length()-2,sb.length()); //remove last ,\n
        sb.append(" })");
        return sb.toString();
    }
}