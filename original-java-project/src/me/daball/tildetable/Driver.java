package me.daball.tildetable;

/**
 * Abstract class Driver - Can be any type of data model driver.
 * 
 * @author David Ball
 */
public abstract class Driver
{
    /**
     * Opens a data connection.
     * 
     * @param  connectionString    any valid connection string, varies by driver
     * @return        true if opened, false otherwise, may throw an Exception if it wants to
     */
    public abstract boolean open(String connectionString);

    /**
     * Closes an open data connection.
     */
    public abstract void close();    

    /**
     * Checks to see if a connection is still alive.
     * 
     * @return        true if opened, false otherwise
     */
    public abstract boolean isOpen();    
    
    /**
     * Runs a query.
     * 
     * @param  queryString  any valid query string, varies by driver
     * @return  Relation or null.
     */
    public abstract Relation query(String queryString);
    
    /**
     * Gets the next row.
     * 
     * @return  Row or null when no more rows.
     */
    public abstract Row nextRow();
    
    /**
     * Rewinds back to the beginning of the result set. If not possible, throw an Exception.
     */
    public abstract void rewind();
}