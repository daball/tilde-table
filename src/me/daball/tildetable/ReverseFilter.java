package me.daball.tildetable;

import java.util.LinkedList;

/**
 * Materializes the result set in memory and reverses the set.
 * 
 * @author David Ball
 */
public class ReverseFilter implements Filter
{
    private LinkedList<Row> data = new LinkedList<Row>();
    private String[] columns;
    private boolean beganReverseSelection = false;
    
    /**
     * Reverses the data set.
     */
    public ReverseFilter(String[] columns)
    {
        this.columns = columns;
    }
    
    /**
     * Copies the filter.
     */
    public ReverseFilter clone()
    {
        ReverseFilter copy = new ReverseFilter(this.columns);
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
     * Returns null on every actual row, and stores that row in a R-B tree.
     * 
     * Once null is reached on the input, it will start returning the sorted rows.
     */
    public Row transformRow(Row row)
    {
        //if a row is not null, as in any valid input rows
        if (row != null)
        {
            //store the input row in a new array and add to the tree by the row key
            this.data.addLast(row);
            
            //echo row output, but don't forget we're going to skip it anyways
            return row;
        }
        //once null starts coming in at the end of the dataset
        else if (row == null)
        {
            //check to see if we've already generated the sorted value collection, if not generate it
            if (!this.beganReverseSelection)
            {
                this.beganReverseSelection = true;
            }

            //get and remove the last item off the LIFO queue
            if (this.data.size() > 0)
            {
                Row endingRow = this.data.getLast();
                this.data.removeLast();
                return endingRow;
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
        if (row != null && !beganReverseSelection)
            return true;
        if (row == null && beganReverseSelection)
            this.beganReverseSelection = false; //reset at end of list
        return false;
    }
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString()
    {
        return "reverse()";
    }
}