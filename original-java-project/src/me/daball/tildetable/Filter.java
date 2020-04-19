package me.daball.tildetable;

/**
 * Filter provides an interface for which to transform data in a Relation as it is processed.
 * 
 * Each filter will be implemented differently, but must provide all these functions in order
 * to permit normal application workflow. If a filter does not intend to change all outputs, then
 * it can simply pass along known-good information.
 * 
 * Be sure to set up your filter constructor to ask good questions about what data it will need
 * in order to process the data correctly. For example, at a minimum, the constructor input will
 * need a current list of column names in order to output getColumns().
 * 
 * @author David Ball
 */
public interface Filter extends Cloneable
{
    /**
     * Transforms the row data as it passes through the filter.
     */
    public Row transformRow(Row row);
    
    /**
     * Transforms the column names as they pass through the filter.
     */
    public String[] getColumns();
    
    /**
     * Transforms the query by letting rows get skipped as they pass through the Relation.
     * Selection filters may use this to completely to omit rows in the result set.
     */
    public boolean isRowSkipped(Row row);
    
    /**
     * Describes the filter in terms of Java.
     */
    public String toString();
    
    /**
     * Copies the filter.
     */
    public Filter clone();
}