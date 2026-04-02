struct Block 
{
    length: u32,
    width : u32,
    height: u32,
}
​
impl Block
{
    fn new(arr: &[u32;3])->Self
    {
        Self
        {
            length: arr[1],
            width : arr[0],
            height: arr[2],  
        }
    }
    
    fn get_length(&self) -> u32
    {
        self.length
    }
    
    fn get_width(&self) -> u32
    {
        self.width
    }
    
    fn get_height(&self) -> u32
    {
        self.height
    }
    
    fn get_volume(&self) -> u32
    {
        self.height * self.width * self.length
    }
    
    fn get_surface_area(&self) -> u32
    {
        2*(self.width * self.length + self.length * self.height + self.width * self.height)
    }
    
}