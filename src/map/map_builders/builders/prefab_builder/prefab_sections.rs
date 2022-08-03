#[derive(PartialEq, Copy, Clone)]
pub enum HorizontalPlacement {
    Left,
    Center,
    Right,
}

#[derive(PartialEq, Copy, Clone)]
pub enum VerticalPlacement {
    Top,
    Center,
    Bottom,
}

#[derive(PartialEq, Copy, Clone)]
pub struct PrefabSection {
    pub template: &'static str,
    pub width: usize,
    pub height: usize,
    pub placement: (HorizontalPlacement, VerticalPlacement),
}

pub const UNDERGROUND_FORT: PrefabSection = PrefabSection {
    template: RIGHT_FORT,
    width: 15,
    height: 43,
    placement: (HorizontalPlacement::Right, VerticalPlacement::Top),
};

#[rustfmt::skip]
// The padding needs to be here!
const RIGHT_FORT : &str = "
     #         
  #######      
  #     #      
  #     #######
  #  g        #
  #     #######
  #     #      
  ### ###      
    # #        
    # #        
    # ##       
    ^          
    ^          
    # ##       
    # #        
    # #        
    # #        
    # #        
  ### ###      
  #     #      
  #     #      
  #  g  #      
  #     #      
  #     #      
  ### ###      
    # #        
    # #        
    # #        
    # ##       
    ^          
    ^          
    # ##       
    # #        
    # #        
    # #        
  ### ###      
  #     #      
  #     #######
  #  g        #
  #     #######
  #     #      
  #######      
     #         
";

pub const ORC_CAMP: PrefabSection = PrefabSection {
    template: ORC_CAMP_TXT,
    width: 12,
    height: 12,
    placement: (HorizontalPlacement::Center, VerticalPlacement::Center),
};

#[rustfmt::skip]
const ORC_CAMP_TXT : &str = "
            
 ########## 
 ≈☼      ☼≈ 
 ≈ g      ≈ 
 ≈        ≈ 
 ≈    g   ≈ 
 o   O    o 
 ≈        ≈ 
 ≈ g      ≈ 
 ≈    g   ≈ 
 ≈☼      ☼≈ 
 ≈≈≈≈o≈≈≈≈≈ 
            
";

pub const DROW_ENTRY: PrefabSection = PrefabSection {
    template: DROW_ENTRY_TXT,
    width: 12,
    height: 10,
    placement: (HorizontalPlacement::Center, VerticalPlacement::Center),
};

#[rustfmt::skip]
const DROW_ENTRY_TXT : &str = "
            
 ########## 
 #        # 
 #   >    # 
 #        # 
 #e       # 
    e     # 
 #e       # 
 ########## 
            
";
