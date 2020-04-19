import java.util.ArrayList;
import java.util.Collection;
import java.util.Iterator;
import java.util.TreeMap;

/**
 * Sorts rows based on the cell input(s) used as a key.
 * This filter *MUST* materialize the view in order to pull off the effect.
 * The view is materialized in memory in a R-B binary tree.
 * Multiple rows with matching keys are stored in an ArrayList on the key node.
 * 
 * @author David Ball
 */
public class SortFilter implements Filter
{
    private TreeMap<String,ArrayList<Row>> sortTree = new TreeMap<String,ArrayList<Row>>();
    private Collection<ArrayList<Row>> sortedCollection;
    private Iterator<ArrayList<Row>> sortedCollectionIterator;
    private Iterator<Row> arrayListIterator;
    
    private String[] actualColumns;
    private String[] indexedColumns;

    //this maps the indexed columns to the actual columns, used to accelerate row selection algorithm during search query
    private Integer[] indexMap;
    
    /**
     * Sorts rows based on the cell input(s) used as a key.
     */
    public SortFilter(String[] columns, String[] indexedColumns) throws Exception
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
    public SortFilter clone()
    {
        try
        {
            SortFilter copy = new SortFilter(this.actualColumns, this.indexedColumns);
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
     * Returns null on every actual row, and stores that row in a R-B tree.
     * 
     * Once null is reached on the input, it will start returning the sorted rows.
     */
    public Row transformRow(Row row)
    {
        //if a row is not null, as in any valid input rows
        if (row != null)
        {
            //get the row key based on the input row
            String rowKey = generateRowKey(row);
                        
            //first check if the key exists in the search tree
            if (this.sortTree.containsKey(rowKey))
            {
                //if the key exists, simply append this row to that node's arraylist
                this.sortTree.get(rowKey).add(row);
            }
            //but if the key doesn't exist in the search tree
            else
            {
                //store the input row in a new array and add to the tree by the row key
                ArrayList<Row> vals = new ArrayList<Row>();
                vals.add(row);
                this.sortTree.put(rowKey, vals);
            }
            
            //echo row output, but don't forget we're going to skip it anyways
            return row;
        }
        //once null starts coming in at the end of the dataset
        else if (row == null)
        {
            //check to see if we've already generated the sorted value collection, if not generate it
            if (this.sortedCollection == null)
            {
                this.sortedCollection = this.sortTree.values();
            }
            
            //check to see if we've already generated the sorted value collection iterator, if not generate it
            if (this.sortedCollectionIterator == null)
            {
                this.sortedCollectionIterator = this.sortedCollection.iterator();
            }
            
            //first look for another item in the last arraylist
            if (arrayListIterator != null && arrayListIterator.hasNext())
            {
                //if so, then return that one recursive fn call at a time
                return this.arrayListIterator.next();
            }
            
            //otherwise walk through the sorted collection iterator, one recursive fn call at a time
            if (this.sortedCollectionIterator.hasNext())
            {
                //store the row at the iterator.next()
                this.arrayListIterator = this.sortedCollectionIterator.next().iterator();
                //then return the first row at sub-iterator
                if (this.arrayListIterator.hasNext())
                {
                    return this.arrayListIterator.next();
                }
            }
        }
        //if no items left in the collection, then return a true null not to be skipped
        return null;
    }
    
    /**
     * Literally skips all rows until null is reached in the data set. Then it doesn't skip any rows.
     */
    public boolean isRowSkipped(Row row)
    {
        //if valid row arrives and we haven't started processing any real output yet
        //then skip the row
        if (row != null && sortedCollection == null)
            return true;
        if (row == null && sortedCollection != null)
        {
            //reset
            this.sortTree.clear();
            this.sortedCollection = null;
            this.sortedCollectionIterator = null;
            this.arrayListIterator = null;
        }
        return false;
    }
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        StringBuilder sb = new StringBuilder();
        sb.append("sort(new String[] {\n");
        for (String column: indexedColumns)
        {
            sb.append("                 \"" + column + "\",\n");
        }
        sb.delete(sb.length()-2,sb.length()); //remove last ,\n
        sb.append(" })");
        return sb.toString();
    }
}