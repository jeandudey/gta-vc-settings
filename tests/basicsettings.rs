/**
 *    Copyright (C) 2016 Jean Piere Dudey
 *
 *    This program is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    This program is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate gta_vc_settings;

use gta_vc_settings::GtaVcSettings;
use std::path::Path;

#[test]
fn read_test_file() {
    let p = Path::new("./tests/gta_vc.set");

    let mut settings = GtaVcSettings::new();
    settings.read(&p);

    assert_eq!(settings.total_sections, 3);
}
